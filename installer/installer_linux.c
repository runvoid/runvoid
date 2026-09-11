// Runvoid Linux Native GUI Installer (x86_64)
// Standalone executable wizard with embedded payload

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <pwd.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

#include "payload_runvoid_linux.h"
#include "payload_gui.h"

static int g_installed = 0;
static int g_progress = 0;
static char g_status_msg[128] = "Click 'Install Runvoid' to begin setup.";

static void get_default_install_dir(char* out, size_t max_len) {
    if (geteuid() == 0) {
        snprintf(out, max_len, "/usr/local/bin");
    } else {
        const char* home = getenv("HOME");
        if (!home) {
            struct passwd* pw = getpwuid(getuid());
            home = pw ? pw->pw_dir : "/tmp";
        }
        snprintf(out, max_len, "%s/.local/bin", home);
    }
}

static void get_default_runtime_dir(char* out, size_t max_len) {
    if (geteuid() == 0) {
        snprintf(out, max_len, "/usr/local/share/runvoid/runtime");
    } else {
        const char* home = getenv("HOME");
        if (!home) {
            struct passwd* pw = getpwuid(getuid());
            home = pw ? pw->pw_dir : "/tmp";
        }
        snprintf(out, max_len, "%s/.local/share/runvoid/runtime", home);
    }
}

static int mkdir_p(const char* dir) {
    char tmp[512];
    char* p = NULL;
    size_t len = strlen(dir);
    if (len >= sizeof(tmp)) return -1;
    strncpy(tmp, dir, sizeof(tmp) - 1);
    tmp[sizeof(tmp) - 1] = '\0';

    for (p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            mkdir(tmp, 0755);
            *p = '/';
        }
    }
    return mkdir(tmp, 0755);
}

static int write_binary_file(const char* path, const unsigned char* data, size_t len, mode_t mode) {
    FILE* f = fopen(path, "wb");
    if (!f) return 0;
    size_t written = fwrite(data, 1, len, f);
    fclose(f);
    if (written == len) {
        chmod(path, mode);
        return 1;
    }
    return 0;
}

static int add_dir_to_path(const char* dir) {
    const char* path_env = getenv("PATH");
    if (path_env && strstr(path_env, dir) != NULL) {
        return 1; // Already in current PATH
    }

    const char* home = getenv("HOME");
    if (!home) return 0;

    const char* config_files[] = {".bashrc", ".zshrc", ".profile"};
    char line[512];
    snprintf(line, sizeof(line), "\n# Runvoid Programming Language\nexport PATH=\"%s:$PATH\"\n", dir);

    for (size_t i = 0; i < 3; i++) {
        char file_path[512];
        snprintf(file_path, sizeof(file_path), "%s/%s", home, config_files[i]);
        if (access(file_path, F_OK) == 0) {
            FILE* f = fopen(file_path, "r");
            if (f) {
                char buf[1024];
                int found = 0;
                while (fgets(buf, sizeof(buf), f)) {
                    if (strstr(buf, dir)) {
                        found = 1;
                        break;
                    }
                }
                fclose(f);
                if (!found) {
                    FILE* fa = fopen(file_path, "a");
                    if (fa) {
                        fputs(line, fa);
                        fclose(fa);
                    }
                }
            }
        }
    }
    return 1;
}

static int execute_installation(const char* dest_dir, const char* runtime_dir) {
    mkdir_p(dest_dir);
    mkdir_p(runtime_dir);

    // Write binary
    char binary_target[512];
    snprintf(binary_target, sizeof(binary_target), "%s/runvoid", dest_dir);
    if (!write_binary_file(binary_target, payload_runvoid_linux_data, payload_runvoid_linux_len, 0755)) {
        return 0;
    }

    // Write runtime gui.c
    char gui_target[512];
    snprintf(gui_target, sizeof(gui_target), "%s/gui.c", runtime_dir);
    write_binary_file(gui_target, payload_gui_data, payload_gui_len, 0644);

    // Update PATH
    add_dir_to_path(dest_dir);
    return 1;
}

static int execute_uninstallation(const char* dest_dir, const char* runtime_dir) {
    char binary_target[512];
    snprintf(binary_target, sizeof(binary_target), "%s/runvoid", dest_dir);
    unlink(binary_target);

    char gui_target[512];
    snprintf(gui_target, sizeof(gui_target), "%s/gui.c", runtime_dir);
    unlink(gui_target);
    return 1;
}

// X11 Color Helper
static unsigned long get_xcolor(Display* dpy, int screen, const char* hex_str) {
    Colormap cmap = DefaultColormap(dpy, screen);
    XColor col;
    if (XParseColor(dpy, cmap, hex_str, &col)) {
        if (XAllocColor(dpy, cmap, &col)) {
            return col.pixel;
        }
    }
    return BlackPixel(dpy, screen);
}

static void run_gui_installer(const char* dest_dir, const char* runtime_dir) {
    Display* dpy = XOpenDisplay(NULL);
    if (!dpy) {
        printf("[WARN] Graphical display ($DISPLAY) not detected. Falling back to CLI mode.\n");
        printf("[INFO] Installing Runvoid to %s ...\n", dest_dir);
        if (execute_installation(dest_dir, runtime_dir)) {
            printf("[PASS] Runvoid v0.3.0 installed successfully to %s/runvoid\n", dest_dir);
        } else {
            printf("[FAIL] Installation failed. Please check permissions.\n");
        }
        return;
    }

    int screen = DefaultScreen(dpy);
    Window root = RootWindow(dpy, screen);

    int win_w = 540;
    int win_h = 390;
    int screen_w = DisplayWidth(dpy, screen);
    int screen_h = DisplayHeight(dpy, screen);
    int win_x = (screen_w - win_w) / 2;
    int win_y = (screen_h - win_h) / 2;

    unsigned long col_bg = get_xcolor(dpy, screen, "#1e1e2e");
    unsigned long col_header = get_xcolor(dpy, screen, "#181825");
    unsigned long col_card = get_xcolor(dpy, screen, "#313244");
    unsigned long col_accent = get_xcolor(dpy, screen, "#89b4fa");
    unsigned long col_btn = get_xcolor(dpy, screen, "#a6e3a1");
    unsigned long col_btn_hover = get_xcolor(dpy, screen, "#94e2d5");
    unsigned long col_text = get_xcolor(dpy, screen, "#cdd6f4");
    unsigned long col_subtext = get_xcolor(dpy, screen, "#a6adc8");
    unsigned long col_prog_bg = get_xcolor(dpy, screen, "#45475a");

    Window win = XCreateSimpleWindow(dpy, root, win_x, win_y, win_w, win_h, 1, col_accent, col_bg);

    XStoreName(dpy, win, "Runvoid v0.3.0 Setup");
    Atom wm_delete = XInternAtom(dpy, "WM_DELETE_WINDOW", False);
    XSetWMProtocols(dpy, win, &wm_delete, 1);

    XSelectInput(dpy, win, ExposureMask | ButtonPressMask | KeyPressMask | StructureNotifyMask);
    XMapWindow(dpy, win);

    GC gc = XCreateGC(dpy, win, 0, NULL);

    int running = 1;
    XEvent ev;

    // Button coordinates
    int btn_x = 330;
    int btn_y = 325;
    int btn_w = 175;
    int btn_h = 36;

    int exit_btn_x = 215;
    int exit_btn_y = 325;
    int exit_btn_w = 100;
    int exit_btn_h = 36;

    while (running) {
        XNextEvent(dpy, &ev);

        if (ev.type == Expose) {
            // 1. Header Banner
            XSetForeground(dpy, gc, col_header);
            XFillRectangle(dpy, win, gc, 0, 0, win_w, 75);

            XSetForeground(dpy, gc, col_accent);
            XDrawLine(dpy, win, gc, 0, 75, win_w, 75);

            XSetForeground(dpy, gc, col_text);
            XDrawString(dpy, win, gc, 25, 32, "Runvoid Programming Language", 28);

            XSetForeground(dpy, gc, col_subtext);
            XDrawString(dpy, win, gc, 25, 55, "Native AOT Compiler & Systems Runtime for Linux x86_64 - v0.3.0", 63);

            // 2. Card: Destination
            XSetForeground(dpy, gc, col_card);
            XFillRectangle(dpy, win, gc, 25, 95, 490, 80);
            XSetForeground(dpy, gc, col_accent);
            XDrawRectangle(dpy, win, gc, 25, 95, 490, 80);

            XSetForeground(dpy, gc, col_text);
            XDrawString(dpy, win, gc, 40, 120, "Installation Target Directory:", 30);

            XSetForeground(dpy, gc, col_header);
            XFillRectangle(dpy, win, gc, 40, 132, 460, 28);
            XSetForeground(dpy, gc, col_accent);
            XDrawString(dpy, win, gc, 50, 150, dest_dir, strlen(dest_dir));

            // 3. Options Info
            XSetForeground(dpy, gc, col_text);
            XDrawString(dpy, win, gc, 30, 200, "[X] Automatically configure user PATH (~/.bashrc, ~/.profile)", 61);
            XDrawString(dpy, win, gc, 30, 222, "[X] Install C runtime components (gui.c)", 40);
            XDrawString(dpy, win, gc, 30, 244, "[X] Set standard permissions (chmod 0755)", 41);

            // 4. Progress Bar
            XSetForeground(dpy, gc, col_prog_bg);
            XFillRectangle(dpy, win, gc, 25, 270, 490, 16);
            if (g_progress > 0) {
                int fill_w = (490 * g_progress) / 100;
                XSetForeground(dpy, gc, col_btn);
                XFillRectangle(dpy, win, gc, 25, 270, fill_w, 16);
            }
            XSetForeground(dpy, gc, col_accent);
            XDrawRectangle(dpy, win, gc, 25, 270, 490, 16);

            // 5. Status text
            XSetForeground(dpy, gc, col_subtext);
            XDrawString(dpy, win, gc, 25, 305, g_status_msg, strlen(g_status_msg));

            // 6. Action Button
            if (g_installed) {
                XSetForeground(dpy, gc, col_btn);
                XFillRectangle(dpy, win, gc, btn_x, btn_y, btn_w, btn_h);
                XSetForeground(dpy, gc, col_header);
                XDrawString(dpy, win, gc, btn_x + 55, btn_y + 22, "Done / Exit", 11);
            } else {
                XSetForeground(dpy, gc, col_btn);
                XFillRectangle(dpy, win, gc, btn_x, btn_y, btn_w, btn_h);
                XSetForeground(dpy, gc, col_header);
                XDrawString(dpy, win, gc, btn_x + 35, btn_y + 22, "Install Runvoid", 15);
            }

            // 7. Exit Button
            if (!g_installed) {
                XSetForeground(dpy, gc, col_card);
                XFillRectangle(dpy, win, gc, exit_btn_x, exit_btn_y, exit_btn_w, exit_btn_h);
                XSetForeground(dpy, gc, col_subtext);
                XDrawRectangle(dpy, win, gc, exit_btn_x, exit_btn_y, exit_btn_w, exit_btn_h);
                XDrawString(dpy, win, gc, exit_btn_x + 28, exit_btn_y + 22, "Cancel", 6);
            }
        } else if (ev.type == ButtonPress) {
            int mx = ev.xbutton.x;
            int my = ev.xbutton.y;

            // Clicked Install / Done button
            if (mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h) {
                if (g_installed) {
                    running = 0;
                } else {
                    snprintf(g_status_msg, sizeof(g_status_msg), "Installing binaries...");
                    g_progress = 40;
                    XClearArea(dpy, win, 0, 0, 0, 0, True);
                    XFlush(dpy);

                    usleep(150000);

                    if (execute_installation(dest_dir, runtime_dir)) {
                        g_progress = 100;
                        g_installed = 1;
                        snprintf(g_status_msg, sizeof(g_status_msg), "Success! Runvoid v0.3.0 is ready to use.");
                    } else {
                        g_progress = 0;
                        snprintf(g_status_msg, sizeof(g_status_msg), "Error: Failed to write files. Check permissions.");
                    }
                    XClearArea(dpy, win, 0, 0, 0, 0, True);
                    XFlush(dpy);
                }
            }
            // Clicked Cancel button
            else if (!g_installed && mx >= exit_btn_x && mx <= exit_btn_x + exit_btn_w && my >= exit_btn_y && my <= exit_btn_y + exit_btn_h) {
                running = 0;
            }
        } else if (ev.type == KeyPress) {
            KeySym sym = XLookupKeysym(&ev.xkey, 0);
            if (sym == XK_Escape) {
                running = 0;
            } else if (sym == XK_Return) {
                if (g_installed) {
                    running = 0;
                } else {
                    g_progress = 100;
                    g_installed = 1;
                    execute_installation(dest_dir, runtime_dir);
                    snprintf(g_status_msg, sizeof(g_status_msg), "Success! Runvoid v0.3.0 is ready.");
                    XClearArea(dpy, win, 0, 0, 0, 0, True);
                }
            }
        } else if (ev.type == ClientMessage) {
            if ((Atom)ev.xclient.data.l[0] == wm_delete) {
                running = 0;
            }
        }
    }

    XFreeGC(dpy, gc);
    XDestroyWindow(dpy, win);
    XCloseDisplay(dpy);
}

int main(int argc, char** argv) {
    char dest_dir[512];
    char runtime_dir[512];
    get_default_install_dir(dest_dir, sizeof(dest_dir));
    get_default_runtime_dir(runtime_dir, sizeof(runtime_dir));

    int cli_mode = 0;
    int uninstall_mode = 0;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--cli") == 0 || strcmp(argv[i], "-c") == 0 || strcmp(argv[i], "--silent") == 0 || strcmp(argv[i], "-s") == 0) {
            cli_mode = 1;
        } else if (strcmp(argv[i], "--uninstall") == 0 || strcmp(argv[i], "-u") == 0) {
            uninstall_mode = 1;
        } else if (strcmp(argv[i], "--dest") == 0 && i + 1 < argc) {
            strncpy(dest_dir, argv[++i], sizeof(dest_dir) - 1);
        }
    }

    if (uninstall_mode) {
        printf("[INFO] Uninstalling Runvoid from %s ...\n", dest_dir);
        execute_uninstallation(dest_dir, runtime_dir);
        printf("[PASS] Runvoid uninstalled successfully.\n");
        return 0;
    }

    if (cli_mode) {
        printf("[INFO] Installing Runvoid (v0.3.0) to %s ...\n", dest_dir);
        if (execute_installation(dest_dir, runtime_dir)) {
            printf("[PASS] Installation complete! Runvoid installed to %s/runvoid\n", dest_dir);
            printf("[INFO] Try running: runvoid --version\n");
            return 0;
        } else {
            fprintf(stderr, "[FAIL] Installation failed. Try running with sudo or check directory permissions.\n");
            return 1;
        }
    }

    run_gui_installer(dest_dir, runtime_dir);
    return 0;
}
