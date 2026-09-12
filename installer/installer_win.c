// Runvoid Windows Native GUI Installer (x86_64)
// Standalone executable wizard with embedded payload

#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <commctrl.h>
#include <shlobj.h>
#include <shlwapi.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "payload_runvoid_win.h"
#include "payload_gui.h"

#define IDC_DEST_EDIT     101
#define IDC_BROWSE_BTN    102
#define IDC_PATH_CHK      103
#define IDC_RUNTIME_CHK   104
#define IDC_PROGRESS      105
#define IDC_STATUS        106
#define IDC_INSTALL_BTN   107
#define IDC_CANCEL_BTN    108

static HWND g_hwnd_main = NULL;
static HWND g_hwnd_edit = NULL;
static HWND g_hwnd_browse = NULL;
static HWND g_hwnd_path_chk = NULL;
static HWND g_hwnd_runtime_chk = NULL;
static HWND g_hwnd_progress = NULL;
static HWND g_hwnd_status = NULL;
static HWND g_hwnd_install = NULL;
static HWND g_hwnd_cancel = NULL;

static HFONT g_font_title = NULL;
static HFONT g_font_ui = NULL;
static HFONT g_font_bold = NULL;

static BOOL g_installed = FALSE;
static char g_dest_path[MAX_PATH] = {0};

static void get_default_install_path(char* out_path, size_t max_len) {
    char local_app_data[MAX_PATH];
    if (SUCCEEDED(SHGetFolderPathA(NULL, CSIDL_LOCAL_APPDATA, NULL, 0, local_app_data))) {
        snprintf(out_path, max_len, "%s\\Programs\\Runvoid\\bin", local_app_data);
    } else {
        snprintf(out_path, max_len, "C:\\Program Files\\Runvoid\\bin");
    }
}

static BOOL create_directories_recursive(const char* dir) {
    char tmp[MAX_PATH];
    char* p = NULL;
    size_t len = strlen(dir);
    if (len >= MAX_PATH) return FALSE;

    strncpy(tmp, dir, sizeof(tmp) - 1);
    tmp[sizeof(tmp) - 1] = '\0';

    for (p = tmp + 1; *p; p++) {
        if (*p == '\\' || *p == '/') {
            *p = '\0';
            CreateDirectoryA(tmp, NULL);
            *p = '\\';
        }
    }
    return CreateDirectoryA(tmp, NULL) || GetLastError() == ERROR_ALREADY_EXISTS;
}

static BOOL write_file_binary(const char* file_path, const unsigned char* data, size_t len) {
    HANDLE hFile = CreateFileA(file_path, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        return FALSE;
    }
    DWORD bytes_written = 0;
    BOOL ok = WriteFile(hFile, data, (DWORD)len, &bytes_written, NULL);
    CloseHandle(hFile);
    return ok && (bytes_written == (DWORD)len);
}

static BOOL add_to_user_path(const char* new_dir) {
    HKEY hKey;
    if (RegOpenKeyExA(HKEY_CURRENT_USER, "Environment", 0, KEY_READ | KEY_WRITE, &hKey) != ERROR_SUCCESS) {
        return FALSE;
    }

    char current_path[32768] = {0};
    DWORD path_len = sizeof(current_path) - 1;
    DWORD val_type = REG_EXPAND_SZ;

    if (RegQueryValueExA(hKey, "Path", NULL, &val_type, (LPBYTE)current_path, &path_len) != ERROR_SUCCESS) {
        current_path[0] = '\0';
    }

    // Check if new_dir is already in path
    if (strstr(current_path, new_dir) != NULL) {
        RegCloseKey(hKey);
        return TRUE;
    }

    char updated_path[32768] = {0};
    if (strlen(current_path) > 0) {
        snprintf(updated_path, sizeof(updated_path), "%s;%s", current_path, new_dir);
    } else {
        snprintf(updated_path, sizeof(updated_path), "%s", new_dir);
    }

    LONG ret = RegSetValueExA(hKey, "Path", 0, REG_EXPAND_SZ, (const BYTE*)updated_path, (DWORD)(strlen(updated_path) + 1));
    RegCloseKey(hKey);

    if (ret == ERROR_SUCCESS) {
        // Broadcast WM_SETTINGCHANGE so running shells can refresh
        DWORD_PTR dwResult;
        SendMessageTimeoutA(HWND_BROADCAST, WM_SETTINGCHANGE, 0, (LPARAM)"Environment", SMTO_ABORTIFHUNG, 3000, &dwResult);
        return TRUE;
    }
    return FALSE;
}

static BOOL remove_from_user_path(const char* dir_to_remove) {
    HKEY hKey;
    if (RegOpenKeyExA(HKEY_CURRENT_USER, "Environment", 0, KEY_READ | KEY_WRITE, &hKey) != ERROR_SUCCESS) {
        return FALSE;
    }

    char current_path[32768] = {0};
    DWORD path_len = sizeof(current_path) - 1;
    DWORD val_type = REG_EXPAND_SZ;

    if (RegQueryValueExA(hKey, "Path", NULL, &val_type, (LPBYTE)current_path, &path_len) != ERROR_SUCCESS) {
        RegCloseKey(hKey);
        return TRUE;
    }

    char* match = strstr(current_path, dir_to_remove);
    if (!match) {
        RegCloseKey(hKey);
        return TRUE;
    }

    char updated_path[32768] = {0};
    size_t prefix_len = match - current_path;
    strncpy(updated_path, current_path, prefix_len);
    updated_path[prefix_len] = '\0';

    size_t match_len = strlen(dir_to_remove);
    const char* suffix = match + match_len;
    if (*suffix == ';') suffix++;

    if (prefix_len > 0 && updated_path[prefix_len - 1] == ';') {
        updated_path[prefix_len - 1] = '\0';
        if (*suffix) {
            strcat(updated_path, ";");
            strcat(updated_path, suffix);
        }
    } else {
        strcat(updated_path, suffix);
    }

    RegSetValueExA(hKey, "Path", 0, REG_EXPAND_SZ, (const BYTE*)updated_path, (DWORD)(strlen(updated_path) + 1));
    RegCloseKey(hKey);

    DWORD_PTR dwResult;
    SendMessageTimeoutA(HWND_BROADCAST, WM_SETTINGCHANGE, 0, (LPARAM)"Environment", SMTO_ABORTIFHUNG, 3000, &dwResult);
    return TRUE;
}

static BOOL perform_installation(HWND hwnd) {
    char target_dir[MAX_PATH] = {0};
    if (hwnd) {
        GetWindowTextA(g_hwnd_edit, target_dir, sizeof(target_dir));
    } else {
        strncpy(target_dir, g_dest_path, sizeof(target_dir));
    }

    if (strlen(target_dir) == 0) {
        if (hwnd) MessageBoxA(hwnd, "Please specify a destination path.", "Error", MB_ICONERROR);
        return FALSE;
    }

    if (hwnd) {
        EnableWindow(g_hwnd_edit, FALSE);
        EnableWindow(g_hwnd_browse, FALSE);
        EnableWindow(g_hwnd_path_chk, FALSE);
        EnableWindow(g_hwnd_runtime_chk, FALSE);
        EnableWindow(g_hwnd_install, FALSE);
        SetWindowTextA(g_hwnd_status, "Creating installation directories...");
        SendMessage(g_hwnd_progress, PBM_SETPOS, 20, 0);
    }

    // 1. Create target directories
    if (!create_directories_recursive(target_dir)) {
        if (hwnd) {
            MessageBoxA(hwnd, "Failed to create installation directory. Please check write permissions.", "Error", MB_ICONERROR);
            SetWindowTextA(g_hwnd_status, "Installation aborted.");
            EnableWindow(g_hwnd_install, TRUE);
        }
        return FALSE;
    }

    char runtime_dir[MAX_PATH];
    snprintf(runtime_dir, sizeof(runtime_dir), "%s\\..\\runtime", target_dir);
    create_directories_recursive(runtime_dir);

    // 2. Extract runvoid.exe
    if (hwnd) {
        SetWindowTextA(g_hwnd_status, "Extracting runvoid.exe compiler binary...");
        SendMessage(g_hwnd_progress, PBM_SETPOS, 50, 0);
    }

    char exe_target[MAX_PATH];
    snprintf(exe_target, sizeof(exe_target), "%s\\runvoid.exe", target_dir);

    if (!write_file_binary(exe_target, payload_runvoid_win_data, payload_runvoid_win_len)) {
        if (hwnd) {
            MessageBoxA(hwnd, "Failed to write runvoid.exe binary.", "Error", MB_ICONERROR);
            SetWindowTextA(g_hwnd_status, "Write failed.");
            EnableWindow(g_hwnd_install, TRUE);
        }
        return FALSE;
    }

    // 3. Extract runtime gui.c
    BOOL install_runtime = hwnd ? (SendMessage(g_hwnd_runtime_chk, BM_GETCHECK, 0, 0) == BST_CHECKED) : TRUE;
    if (install_runtime) {
        if (hwnd) {
            SetWindowTextA(g_hwnd_status, "Installing C runtime components (gui.c)...");
            SendMessage(g_hwnd_progress, PBM_SETPOS, 75, 0);
        }
        char gui_target[MAX_PATH];
        snprintf(gui_target, sizeof(gui_target), "%s\\gui.c", runtime_dir);
        write_file_binary(gui_target, payload_gui_data, payload_gui_len);
    }

    // 4. Update PATH
    BOOL add_path = hwnd ? (SendMessage(g_hwnd_path_chk, BM_GETCHECK, 0, 0) == BST_CHECKED) : TRUE;
    if (add_path) {
        if (hwnd) {
            SetWindowTextA(g_hwnd_status, "Configuring system PATH environment variable...");
            SendMessage(g_hwnd_progress, PBM_SETPOS, 90, 0);
        }
        add_to_user_path(target_dir);
    }

    // 5. Complete
    if (hwnd) {
        SendMessage(g_hwnd_progress, PBM_SETPOS, 100, 0);
        SetWindowTextA(g_hwnd_status, "Installation complete!");
        SetWindowTextA(g_hwnd_install, "Finish");
        EnableWindow(g_hwnd_install, TRUE);
        g_installed = TRUE;

        MessageBoxA(hwnd,
            "Runvoid 1 (v1.3.0) has been installed successfully!\n\n"
            "You can now open a new Command Prompt or PowerShell window\n"
            "and run:\n\n"
            "    runvoid --version\n"
            "    runvoid cheat",
            "Setup Complete",
            MB_ICONINFORMATION);
    }

    return TRUE;
}

static BOOL perform_uninstallation(const char* target_dir) {
    char exe_target[MAX_PATH];
    snprintf(exe_target, sizeof(exe_target), "%s\\runvoid.exe", target_dir);
    DeleteFileA(exe_target);

    char gui_target[MAX_PATH];
    snprintf(gui_target, sizeof(gui_target), "%s\\..\\runtime\\gui.c", target_dir);
    DeleteFileA(gui_target);

    remove_from_user_path(target_dir);
    return TRUE;
}

static LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch (msg) {
        case WM_CREATE: {
            g_hwnd_main = hwnd;

            // Header Banner
            CreateWindowExA(0, "STATIC", "Runvoid Programming Language",
                WS_CHILD | WS_VISIBLE | SS_LEFT,
                24, 20, 460, 26, hwnd, NULL, NULL, NULL);

            CreateWindowExA(0, "STATIC", "Native Compiler & Systems Runtime for Windows x86_64 — Runvoid 1 (v1.3.0)",
                WS_CHILD | WS_VISIBLE | SS_LEFT,
                24, 48, 460, 20, hwnd, NULL, NULL, NULL);

            // Group Box
            CreateWindowExA(0, "BUTTON", "Installation Destination",
                WS_CHILD | WS_VISIBLE | BS_GROUPBOX,
                20, 80, 480, 75, hwnd, NULL, NULL, NULL);

            // Destination Edit
            g_hwnd_edit = CreateWindowExA(WS_EX_CLIENTEDGE, "EDIT", g_dest_path,
                WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL,
                35, 110, 360, 24, hwnd, (HMENU)IDC_DEST_EDIT, NULL, NULL);

            // Browse Button
            g_hwnd_browse = CreateWindowExA(0, "BUTTON", "Browse...",
                WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                405, 110, 80, 24, hwnd, (HMENU)IDC_BROWSE_BTN, NULL, NULL);

            // Checkboxes
            g_hwnd_path_chk = CreateWindowExA(0, "BUTTON", "Add Runvoid to User PATH environment variable (Recommended)",
                WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
                24, 170, 460, 22, hwnd, (HMENU)IDC_PATH_CHK, NULL, NULL);
            SendMessage(g_hwnd_path_chk, BM_SETCHECK, BST_CHECKED, 0);

            g_hwnd_runtime_chk = CreateWindowExA(0, "BUTTON", "Install C standard runtime libraries (gui.c)",
                WS_CHILD | WS_VISIBLE | BS_AUTOCHECKBOX,
                24, 196, 460, 22, hwnd, (HMENU)IDC_RUNTIME_CHK, NULL, NULL);
            SendMessage(g_hwnd_runtime_chk, BM_SETCHECK, BST_CHECKED, 0);

            // Progress Bar
            g_hwnd_progress = CreateWindowExA(0, PROGRESS_CLASSA, NULL,
                WS_CHILD | WS_VISIBLE | PBS_SMOOTH,
                24, 235, 472, 18, hwnd, (HMENU)IDC_PROGRESS, NULL, NULL);
            SendMessage(g_hwnd_progress, PBM_SETRANGE, 0, MAKELPARAM(0, 100));
            SendMessage(g_hwnd_progress, PBM_SETPOS, 0, 0);

            // Status Label
            g_hwnd_status = CreateWindowExA(0, "STATIC", "Click Install to begin setup.",
                WS_CHILD | WS_VISIBLE | SS_LEFT,
                24, 260, 472, 20, hwnd, (HMENU)IDC_STATUS, NULL, NULL);

            // Buttons
            g_hwnd_install = CreateWindowExA(0, "BUTTON", "Install",
                WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
                300, 300, 100, 30, hwnd, (HMENU)IDC_INSTALL_BTN, NULL, NULL);

            g_hwnd_cancel = CreateWindowExA(0, "BUTTON", "Cancel",
                WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                410, 300, 86, 30, hwnd, (HMENU)IDC_CANCEL_BTN, NULL, NULL);

            // Apply Fonts
            g_font_title = CreateFontA(20, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE,
                DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                DEFAULT_QUALITY, DEFAULT_PITCH | FF_SWISS, "Segoe UI");
            g_font_ui = CreateFontA(14, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                DEFAULT_QUALITY, DEFAULT_PITCH | FF_SWISS, "Segoe UI");

            EnumChildWindows(hwnd, (WNDENUMPROC)(void*)SendMessage, (LPARAM)WM_SETFONT);
            return 0;
        }

        case WM_COMMAND: {
            int wmId = LOWORD(wParam);
            if (wmId == IDC_BROWSE_BTN) {
                BROWSEINFOA bi = {0};
                bi.hwndOwner = hwnd;
                bi.lpszTitle = "Select Runvoid Installation Folder:";
                bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;
                LPITEMIDLIST pidl = SHBrowseForFolderA(&bi);
                if (pidl) {
                    char selected[MAX_PATH];
                    if (SHGetPathFromIDListA(pidl, selected)) {
                        SetWindowTextA(g_hwnd_edit, selected);
                    }
                    CoTaskMemFree(pidl);
                }
            } else if (wmId == IDC_INSTALL_BTN) {
                if (g_installed) {
                    PostQuitMessage(0);
                } else {
                    perform_installation(hwnd);
                }
            } else if (wmId == IDC_CANCEL_BTN) {
                PostQuitMessage(0);
            }
            return 0;
        }

        case WM_DESTROY: {
            if (g_font_title) DeleteObject(g_font_title);
            if (g_font_ui) DeleteObject(g_font_ui);
            PostQuitMessage(0);
            return 0;
        }
    }
    return DefWindowProcA(hwnd, msg, wParam, lParam);
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    (void)hPrevInstance;
    (void)nCmdShow;

    // Check command line arguments for silent or uninstall modes
    BOOL silent_mode = FALSE;
    BOOL uninstall_mode = FALSE;

    get_default_install_path(g_dest_path, sizeof(g_dest_path));

    if (lpCmdLine && strlen(lpCmdLine) > 0) {
        if (strstr(lpCmdLine, "/S") || strstr(lpCmdLine, "--silent")) {
            silent_mode = TRUE;
        }
        if (strstr(lpCmdLine, "/U") || strstr(lpCmdLine, "--uninstall")) {
            uninstall_mode = TRUE;
        }
        char* dest_arg = strstr(lpCmdLine, "/D=");
        if (dest_arg) {
            strncpy(g_dest_path, dest_arg + 3, sizeof(g_dest_path) - 1);
        }
    }

    if (uninstall_mode) {
        perform_uninstallation(g_dest_path);
        if (!silent_mode) {
            MessageBoxA(NULL, "Runvoid has been uninstalled successfully.", "Runvoid Uninstaller", MB_ICONINFORMATION);
        }
        return 0;
    }

    if (silent_mode) {
        return perform_installation(NULL) ? 0 : 1;
    }

    // Initialize Common Controls (Progress bar, styling)
    INITCOMMONCONTROLSEX icex;
    icex.dwSize = sizeof(INITCOMMONCONTROLSEX);
    icex.dwICC = ICC_PROGRESS_CLASS | ICC_STANDARD_CLASSES;
    InitCommonControlsEx(&icex);

    // Register Window Class
    WNDCLASSEXA wc = {0};
    wc.cbSize = sizeof(WNDCLASSEXA);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);
    wc.hbrBackground = (HBRUSH)(COLOR_BTNFACE + 1);
    wc.lpszClassName = "RunvoidSetupWindowClass";

    if (!RegisterClassExA(&wc)) {
        MessageBoxA(NULL, "Window Registration Failed!", "Error", MB_ICONEXCLAMATION | MB_OK);
        return 1;
    }

    // Center Window on Screen
    int win_w = 530;
    int win_h = 390;
    int screen_w = GetSystemMetrics(SM_CXSCREEN);
    int screen_h = GetSystemMetrics(SM_CYSCREEN);
    int pos_x = (screen_w - win_w) / 2;
    int pos_y = (screen_h - win_h) / 2;

    HWND hwnd = CreateWindowExA(
        WS_EX_APPWINDOW,
        "RunvoidSetupWindowClass",
        "Runvoid 1 (v1.3.0) Setup",
        WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
        pos_x, pos_y, win_w, win_h,
        NULL, NULL, hInstance, NULL);

    if (!hwnd) {
        MessageBoxA(NULL, "Window Creation Failed!", "Error", MB_ICONEXCLAMATION | MB_OK);
        return 1;
    }

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    MSG msg;
    while (GetMessageA(&msg, NULL, 0, 0) > 0) {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }

    return (int)msg.wParam;
}
