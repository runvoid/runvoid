#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <time.h>
#include <math.h>
#include <pthread.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#ifndef NO_GUI
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#endif

typedef struct {
    uint64_t len;
    char data[1];
} RvString;

typedef struct {
    char label[128];
    int x, y, w, h;
    void (*callback)(void);
} GuiButton;

typedef struct {
    char text[256];
    int x, y;
} GuiLabel;

typedef struct {
    char label[128];
    int x, y, w, h;
    int checked;
} GuiCheckbox;

#ifdef NO_GUI
void rv_gui_init(RvString *title_obj, int64_t width, int64_t height) { (void)title_obj; (void)width; (void)height; }
void rv_gui_add_label(RvString *text_obj) { (void)text_obj; }
void rv_gui_add_button(RvString *label_obj, void (*callback)(void)) { (void)label_obj; (void)callback; }
void rv_gui_add_checkbox(RvString *label_obj, int64_t initial_val) { (void)label_obj; (void)initial_val; }
void rv_gui_loop(void) {}
#else
static Display *disp = NULL;
static Window win;
static GC gc;
static int screen_num;
static unsigned long white_pixel, black_pixel, blue_pixel, gray_pixel;

static GuiButton buttons[32];
static int num_buttons = 0;

static GuiLabel labels[32];
static int num_labels = 0;

static GuiCheckbox checkboxes[32];
static int num_checkboxes = 0;

static int current_y = 20;

void rv_gui_init(RvString *title_obj, int64_t width, int64_t height) {
    if (width <= 0) width = 500;
    if (height <= 0) height = 400;

    const char *title = title_obj ? (const char*)&title_obj->data[0] : "Runvoid Window";

    disp = XOpenDisplay(NULL);
    if (!disp) {
        printf("--- [Runvoid GUI Mode: %s (%dx%d)] ---\n", title, (int)width, (int)height);
        return;
    }

    screen_num = DefaultScreen(disp);
    white_pixel = WhitePixel(disp, screen_num);
    black_pixel = BlackPixel(disp, screen_num);

    Colormap colormap = DefaultColormap(disp, screen_num);
    XColor blue_col, gray_col;
    XAllocNamedColor(disp, colormap, "#2563EB", &blue_col, &blue_col);
    XAllocNamedColor(disp, colormap, "#E5E7EB", &gray_col, &gray_col);
    blue_pixel = blue_col.pixel;
    gray_pixel = gray_col.pixel;

    win = XCreateSimpleWindow(disp, RootWindow(disp, screen_num), 100, 100, width, height, 2, black_pixel, white_pixel);
    XStoreName(disp, win, title);
    XSelectInput(disp, win, ExposureMask | KeyPressMask | ButtonPressMask);
    XMapWindow(disp, win);

    gc = XCreateGC(disp, win, 0, NULL);
    XFlush(disp);
}

void rv_gui_add_label(RvString *text_obj) {
    if (!text_obj) return;
    if (num_labels < 32) {
        strncpy(labels[num_labels].text, (const char*)&text_obj->data[0], 255);
        labels[num_labels].x = 30;
        labels[num_labels].y = current_y + 15;
        num_labels++;
        current_y += 35;
    }
}

void rv_gui_add_button(RvString *label_obj, void (*cb)(void)) {
    if (!label_obj) return;
    if (num_buttons < 32) {
        strncpy(buttons[num_buttons].label, (const char*)&label_obj->data[0], 127);
        buttons[num_buttons].x = 30;
        buttons[num_buttons].y = current_y;
        buttons[num_buttons].w = 160;
        buttons[num_buttons].h = 32;
        buttons[num_buttons].callback = cb;
        num_buttons++;
        current_y += 45;
    }
}

void rv_gui_add_checkbox(RvString *label_obj, int64_t initial_val) {
    if (!label_obj) return;
    if (num_checkboxes < 32) {
        strncpy(checkboxes[num_checkboxes].label, (const char*)&label_obj->data[0], 127);
        checkboxes[num_checkboxes].x = 30;
        checkboxes[num_checkboxes].y = current_y;
        checkboxes[num_checkboxes].w = 200;
        checkboxes[num_checkboxes].h = 24;
        checkboxes[num_checkboxes].checked = (int)initial_val;
        num_checkboxes++;
        current_y += 35;
    }
}

static void redraw_all() {
    if (!disp) return;

    // Draw labels
    XSetForeground(disp, gc, black_pixel);
    for (int i = 0; i < num_labels; i++) {
        XDrawString(disp, win, gc, labels[i].x, labels[i].y, labels[i].text, strlen(labels[i].text));
    }

    // Draw buttons
    for (int i = 0; i < num_buttons; i++) {
        XSetForeground(disp, gc, gray_pixel);
        XFillRectangle(disp, win, gc, buttons[i].x, buttons[i].y, buttons[i].w, buttons[i].h);
        XSetForeground(disp, gc, blue_pixel);
        XDrawRectangle(disp, win, gc, buttons[i].x, buttons[i].y, buttons[i].w, buttons[i].h);

        XSetForeground(disp, gc, black_pixel);
        XDrawString(disp, win, gc, buttons[i].x + 15, buttons[i].y + 20, buttons[i].label, strlen(buttons[i].label));
    }

    // Draw checkboxes
    for (int i = 0; i < num_checkboxes; i++) {
        XSetForeground(disp, gc, white_pixel);
        XFillRectangle(disp, win, gc, checkboxes[i].x, checkboxes[i].y, 16, 16);
        XSetForeground(disp, gc, black_pixel);
        XDrawRectangle(disp, win, gc, checkboxes[i].x, checkboxes[i].y, 16, 16);

        if (checkboxes[i].checked) {
            XSetForeground(disp, gc, blue_pixel);
            XFillRectangle(disp, win, gc, checkboxes[i].x + 3, checkboxes[i].y + 3, 10, 10);
        }

        XSetForeground(disp, gc, black_pixel);
        XDrawString(disp, win, gc, checkboxes[i].x + 25, checkboxes[i].y + 13, checkboxes[i].label, strlen(checkboxes[i].label));
    }
    XFlush(disp);
}

void rv_gui_loop() {
    if (!disp) {
        printf("Running in terminal GUI mode. Window closed.\n");
        return;
    }

    XEvent ev;
    int running = 1;

    redraw_all();

    while (running) {
        XNextEvent(disp, &ev);
        if (ev.type == Expose) {
            redraw_all();
        } else if (ev.type == ButtonPress) {
            int mx = ev.xbutton.x;
            int my = ev.xbutton.y;

            // Check buttons
            for (int i = 0; i < num_buttons; i++) {
                if (mx >= buttons[i].x && mx <= buttons[i].x + buttons[i].w &&
                    my >= buttons[i].y && my <= buttons[i].y + buttons[i].h) {
                    if (buttons[i].callback) {
                        buttons[i].callback();
                    }
                }
            }

            // Check checkboxes
            for (int i = 0; i < num_checkboxes; i++) {
                if (mx >= checkboxes[i].x && mx <= checkboxes[i].x + checkboxes[i].w &&
                    my >= checkboxes[i].y && my <= checkboxes[i].y + checkboxes[i].h) {
                    checkboxes[i].checked = !checkboxes[i].checked;
                    redraw_all();
                }
            }
        } else if (ev.type == KeyPress) {
            // Close on Escape or 'q'
            KeySym key = XLookupKeysym(&ev.xkey, 0);
            if (key == XK_Escape || key == XK_q) {
                running = 0;
            }
        }
    }

    XFreeGC(disp, gc);
    XDestroyWindow(disp, win);
    XCloseDisplay(disp);
}
#endif

// ----------------------------------------------------------------------------
// Immediate Mode GUI (imrv) functions
// ----------------------------------------------------------------------------
int64_t rv_imrv_draw_checkbox(RvString *label_obj, int64_t state) {
    const char *lbl = label_obj ? (const char*)&label_obj->data[0] : "";
    int new_state = !state;
    printf("[imrv] Checkbox '%s': %s -> %s\n", lbl, state ? "CHECKED" : "UNCHECKED", new_state ? "CHECKED" : "UNCHECKED");
    return new_state;
}

int64_t rv_imrv_draw_button(RvString *label_obj) {
    const char *lbl = label_obj ? (const char*)&label_obj->data[0] : "";
    printf("[imrv] Button '%s' clicked!\n", lbl);
    return 1;
}

void rv_imrv_draw_text(RvString *text_obj) {
    const char *txt = text_obj ? (const char*)&text_obj->data[0] : "";
    printf("[imrv] Text: %s\n", txt);
}

// ----------------------------------------------------------------------------
// Beginner & Systems Runtime Helpers
// ----------------------------------------------------------------------------
#include <stdarg.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <time.h>

static RvString* rv_cstr_to_str(const char *s) {
    if (!s) s = "";
    uint64_t len = strlen(s);
    RvString *obj = (RvString*)malloc(sizeof(uint64_t) + len + 1);
    obj->len = len;
    memcpy(obj->data, s, len);
    obj->data[len] = '\0';
    return obj;
}

// Colored Output
void rv_say_color_str(RvString *str, int64_t color_code, int64_t newline) {
    const char *color_seq = "\033[0m";
    switch (color_code) {
        case 1: color_seq = "\033[31m"; break; // red
        case 2: color_seq = "\033[32m"; break; // green
        case 3: color_seq = "\033[33m"; break; // yellow
        case 4: color_seq = "\033[34m"; break; // blue
        case 5: color_seq = "\033[35m"; break; // magenta
        case 6: color_seq = "\033[36m"; break; // cyan
    }
    const char *text = str ? (const char*)&str->data[0] : "";
    if (newline) {
        printf("%s%s\033[0m\n", color_seq, text);
    } else {
        printf("%s%s\033[0m", color_seq, text);
    }
    fflush(stdout);
}

// Dialogs & Sound
void rv_alert(RvString *msg) {
    const char *text = msg ? (const char*)&msg->data[0] : "";
    char cmd[1024];
    if (getenv("DISPLAY") || getenv("WAYLAND_DISPLAY")) {
        snprintf(cmd, sizeof(cmd), "zenity --info --text=\"%s\" 2>/dev/null || xmessage -center \"%s\" 2>/dev/null", text, text);
        if (system(cmd) == 0) return;
    }
    printf("\n╔══════════════════════════════════════╗\n");
    printf("║ ALERT: %-29s ║\n", text);
    printf("╚══════════════════════════════════════╝\n\n");
}

int64_t rv_ask_user(RvString *prompt) {
    const char *text = prompt ? (const char*)&prompt->data[0] : "Confirm?";
    char cmd[1024];
    if (getenv("DISPLAY") || getenv("WAYLAND_DISPLAY")) {
        snprintf(cmd, sizeof(cmd), "zenity --question --text=\"%s\" 2>/dev/null", text);
        int res = system(cmd);
        if (WIFEXITED(res) && WEXITSTATUS(res) == 0) return 1;
        if (WIFEXITED(res) && WEXITSTATUS(res) == 1) return 0;
    }
    printf("%s [y/N]: ", text);
    fflush(stdout);
    char buf[64];
    if (fgets(buf, sizeof(buf), stdin)) {
        if (buf[0] == 'y' || buf[0] == 'Y') return 1;
    }
    return 0;
}

void rv_beep(void) {
    printf("\a");
    fflush(stdout);
}

void rv_speak(RvString *msg) {
    const char *text = msg ? (const char*)&msg->data[0] : "";
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "spd-say \"%s\" 2>/dev/null || espeak \"%s\" 2>/dev/null || (echo \"[Speech: %s]\")", text, text, text);
    int _ = system(cmd);
    (void)_;
}

// Web
void rv_open_web(RvString *url) {
    const char *link = url ? (const char*)&url->data[0] : "";
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "xdg-open \"%s\" >/dev/null 2>&1 &", link);
    int _ = system(cmd);
    (void)_;
}

void rv_download(RvString *url, RvString *path) {
    const char *u = url ? (const char*)&url->data[0] : "";
    const char *p = path ? (const char*)&path->data[0] : "";
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "curl -sL \"%s\" -o \"%s\" 2>/dev/null || wget -q \"%s\" -O \"%s\" 2>/dev/null", u, p, u, p);
    int _ = system(cmd);
    (void)_;
}

RvString* rv_read_web(RvString *url) {
    const char *u = url ? (const char*)&url->data[0] : "";
    char cmd[1024];
    snprintf(cmd, sizeof(cmd), "curl -sL \"%s\" 2>/dev/null || wget -qO- \"%s\" 2>/dev/null", u, u);
    FILE *fp = popen(cmd, "r");
    if (!fp) return rv_cstr_to_str("");
    char buffer[65536];
    size_t total = 0;
    size_t n;
    while ((n = fread(buffer + total, 1, sizeof(buffer) - total - 1, fp)) > 0) {
        total += n;
        if (total >= sizeof(buffer) - 1) break;
    }
    buffer[total] = '\0';
    pclose(fp);
    return rv_cstr_to_str(buffer);
}

// Terminal Console
void rv_clear_screen(void) {
    printf("\033[2J\033[H");
    fflush(stdout);
}

void rv_cursor_at(int64_t x, int64_t y) {
    if (x < 1) x = 1;
    if (y < 1) y = 1;
    printf("\033[%ld;%ldH", y, x);
    fflush(stdout);
}

RvString* rv_ask_hidden(RvString *prompt) {
    const char *p = prompt ? (const char*)&prompt->data[0] : "Password: ";
    char *pass = getpass(p);
    return rv_cstr_to_str(pass ? pass : "");
}

RvString* rv_choose(RvString *prompt, int64_t count, ...) {
    const char *title = prompt ? (const char*)&prompt->data[0] : "Choose an option:";
    printf("\n%s\n", title);
    va_list args;
    va_start(args, count);
    RvString *opts[32];
    for (int64_t i = 0; i < count && i < 32; i++) {
        opts[i] = va_arg(args, RvString*);
        const char *opt_text = opts[i] ? (const char*)&opts[i]->data[0] : "";
        printf("  [%ld] %s\n", i + 1, opt_text);
    }
    va_end(args);
    printf("Enter choice (1-%ld): ", count);
    fflush(stdout);
    char buf[64];
    int chosen = 1;
    if (fgets(buf, sizeof(buf), stdin)) {
        int v = atoi(buf);
        if (v >= 1 && v <= count) {
            chosen = v;
        }
    }
    return opts[chosen - 1] ? opts[chosen - 1] : rv_cstr_to_str("");
}

// File & Folder Operations
void rv_create_folder(RvString *path) {
    const char *p = path ? (const char*)&path->data[0] : "";
    mkdir(p, 0755);
}

void rv_delete_file(RvString *path) {
    const char *p = path ? (const char*)&path->data[0] : "";
    unlink(p);
}

void rv_delete_folder(RvString *path) {
    const char *p = path ? (const char*)&path->data[0] : "";
    rmdir(p);
}

void rv_copy_file(RvString *src, RvString *dst) {
    const char *s = src ? (const char*)&src->data[0] : "";
    const char *d = dst ? (const char*)&dst->data[0] : "";
    FILE *fsrc = fopen(s, "rb");
    if (!fsrc) return;
    FILE *fdst = fopen(d, "wb");
    if (!fdst) { fclose(fsrc); return; }
    char buf[8192];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), fsrc)) > 0) {
        fwrite(buf, 1, n, fdst);
    }
    fclose(fsrc);
    fclose(fdst);
}

int64_t rv_file_exists(RvString *path) {
    const char *p = path ? (const char*)&path->data[0] : "";
    return access(p, F_OK) == 0 ? 1 : 0;
}

// String Operations
RvString* rv_str_replace(RvString *target, RvString *rep, RvString *src) {
    const char *t = target ? (const char*)&target->data[0] : "";
    const char *r = rep ? (const char*)&rep->data[0] : "";
    const char *s = src ? (const char*)&src->data[0] : "";
    if (!*t) return rv_cstr_to_str(s);

    char result[65536];
    result[0] = '\0';
    size_t t_len = strlen(t);
    size_t r_len = strlen(r);
    const char *p = s;
    char *out = result;
    size_t remaining = sizeof(result) - 1;

    while (*p && remaining > 0) {
        if (strncmp(p, t, t_len) == 0) {
            if (r_len < remaining) {
                memcpy(out, r, r_len);
                out += r_len;
                remaining -= r_len;
                p += t_len;
            } else {
                break;
            }
        } else {
            *out++ = *p++;
            remaining--;
        }
    }
    *out = '\0';
    return rv_cstr_to_str(result);
}

RvString* rv_str_upper(RvString *src) {
    const char *s = src ? (const char*)&src->data[0] : "";
    char buf[65536];
    size_t len = strlen(s);
    if (len >= sizeof(buf)) len = sizeof(buf) - 1;
    for (size_t i = 0; i < len; i++) {
        char c = s[i];
        if (c >= 'a' && c <= 'z') c = c - 'a' + 'A';
        buf[i] = c;
    }
    buf[len] = '\0';
    return rv_cstr_to_str(buf);
}

RvString* rv_str_lower(RvString *src) {
    const char *s = src ? (const char*)&src->data[0] : "";
    char buf[65536];
    size_t len = strlen(s);
    if (len >= sizeof(buf)) len = sizeof(buf) - 1;
    for (size_t i = 0; i < len; i++) {
        char c = s[i];
        if (c >= 'A' && c <= 'Z') c = c - 'A' + 'a';
        buf[i] = c;
    }
    buf[len] = '\0';
    return rv_cstr_to_str(buf);
}

RvString* rv_str_trim(RvString *src) {
    const char *s = src ? (const char*)&src->data[0] : "";
    while (*s == ' ' || *s == '\t' || *s == '\r' || *s == '\n') s++;
    char buf[65536];
    strncpy(buf, s, sizeof(buf) - 1);
    buf[sizeof(buf) - 1] = '\0';
    size_t len = strlen(buf);
    while (len > 0 && (buf[len - 1] == ' ' || buf[len - 1] == '\t' || buf[len - 1] == '\r' || buf[len - 1] == '\n')) {
        buf[len - 1] = '\0';
        len--;
    }
    return rv_cstr_to_str(buf);
}

int64_t rv_str_starts_with(RvString *src, RvString *prefix) {
    const char *s = src ? (const char*)&src->data[0] : "";
    const char *p = prefix ? (const char*)&prefix->data[0] : "";
    size_t p_len = strlen(p);
    return strncmp(s, p, p_len) == 0 ? 1 : 0;
}

int64_t rv_str_ends_with(RvString *src, RvString *suffix) {
    const char *s = src ? (const char*)&src->data[0] : "";
    const char *p = suffix ? (const char*)&suffix->data[0] : "";
    size_t s_len = strlen(s);
    size_t p_len = strlen(p);
    if (p_len > s_len) return 0;
    return strcmp(s + s_len - p_len, p) == 0 ? 1 : 0;
}

// 2D Screen Canvas
#ifdef NO_GUI
void rv_screen_init(RvString *title, int64_t width, int64_t height) { (void)title; (void)width; (void)height; }
void rv_screen_draw_rect(int64_t x, int64_t y, int64_t w, int64_t h, RvString *color) { (void)x; (void)y; (void)w; (void)h; (void)color; }
void rv_screen_draw_circle(int64_t x, int64_t y, int64_t r, RvString *color) { (void)x; (void)y; (void)r; (void)color; }
void rv_screen_draw_line(int64_t x1, int64_t y1, int64_t x2, int64_t y2, RvString *color) { (void)x1; (void)y1; (void)x2; (void)y2; (void)color; }
void rv_screen_draw_text(int64_t x, int64_t y, RvString *text, RvString *color) { (void)x; (void)y; (void)text; (void)color; }
void rv_screen_loop(void) {}
#else
typedef struct {
    int type; // 1 = rect, 2 = circle, 3 = line, 4 = text
    int x1, y1, x2, y2, r;
    char text[128];
    char color[32];
} ScreenShape;

static ScreenShape screen_shapes[256];
static int num_screen_shapes = 0;

void rv_screen_init(RvString *title, int64_t width, int64_t height) {
    rv_gui_init(title, width, height);
}

void rv_screen_draw_rect(int64_t x, int64_t y, int64_t w, int64_t h, RvString *color) {
    if (num_screen_shapes < 256) {
        ScreenShape *s = &screen_shapes[num_screen_shapes++];
        s->type = 1;
        s->x1 = x; s->y1 = y; s->x2 = w; s->y2 = h;
        strncpy(s->color, color ? (const char*)&color->data[0] : "black", 31);
    }
}

void rv_screen_draw_circle(int64_t x, int64_t y, int64_t r, RvString *color) {
    if (num_screen_shapes < 256) {
        ScreenShape *s = &screen_shapes[num_screen_shapes++];
        s->type = 2;
        s->x1 = x; s->y1 = y; s->r = r;
        strncpy(s->color, color ? (const char*)&color->data[0] : "black", 31);
    }
}

void rv_screen_draw_line(int64_t x1, int64_t y1, int64_t x2, int64_t y2, RvString *color) {
    if (num_screen_shapes < 256) {
        ScreenShape *s = &screen_shapes[num_screen_shapes++];
        s->type = 3;
        s->x1 = x1; s->y1 = y1; s->x2 = x2; s->y2 = y2;
        strncpy(s->color, color ? (const char*)&color->data[0] : "black", 31);
    }
}

void rv_screen_draw_text(int64_t x, int64_t y, RvString *text, RvString *color) {
    if (num_screen_shapes < 256) {
        ScreenShape *s = &screen_shapes[num_screen_shapes++];
        s->type = 4;
        s->x1 = x; s->y1 = y;
        strncpy(s->text, text ? (const char*)&text->data[0] : "", 127);
        strncpy(s->color, color ? (const char*)&color->data[0] : "black", 31);
    }
}

void rv_screen_loop(void) {
    if (!disp) {
        printf("--- [Runvoid 2D Screen: %d shapes rendered] ---\n", num_screen_shapes);
        return;
    }

    XEvent ev;
    int running = 1;

    while (running) {
        XNextEvent(disp, &ev);
        if (ev.type == Expose) {
            Colormap colormap = DefaultColormap(disp, screen_num);
            for (int i = 0; i < num_screen_shapes; i++) {
                ScreenShape *s = &screen_shapes[i];
                XColor xcol;
                unsigned long pix = black_pixel;
                if (XAllocNamedColor(disp, colormap, s->color, &xcol, &xcol)) {
                    pix = xcol.pixel;
                }
                XSetForeground(disp, gc, pix);
                if (s->type == 1) {
                    XFillRectangle(disp, win, gc, s->x1, s->y1, s->x2, s->y2);
                } else if (s->type == 2) {
                    XFillArc(disp, win, gc, s->x1 - s->r, s->y1 - s->r, s->r * 2, s->r * 2, 0, 360 * 64);
                } else if (s->type == 3) {
                    XDrawLine(disp, win, gc, s->x1, s->y1, s->x2, s->y2);
                } else if (s->type == 4) {
                    XDrawString(disp, win, gc, s->x1, s->y1, s->text, strlen(s->text));
                }
            }
            XFlush(disp);
        } else if (ev.type == KeyPress || ev.type == ButtonPress) {
            running = 0;
        }
    }

    XFreeGC(disp, gc);
    XDestroyWindow(disp, win);
    XCloseDisplay(disp);
}
#endif

// Lists & Collections
typedef struct {
    int64_t count;
    int64_t cap;
    RvString **items;
} RvList;

RvList* rv_list_create(int64_t initial_cap) {
    if (initial_cap <= 0) initial_cap = 8;
    RvList *list = (RvList*)malloc(sizeof(RvList));
    list->count = 0;
    list->cap = initial_cap;
    list->items = (RvString**)malloc(sizeof(RvString*) * initial_cap);
    return list;
}

void rv_list_add(RvList *list, RvString *item) {
    if (!list) return;
    if (list->count >= list->cap) {
        list->cap *= 2;
        list->items = (RvString**)realloc(list->items, sizeof(RvString*) * list->cap);
    }
    list->items[list->count++] = item;
}

void rv_list_remove(RvList *list, RvString *item) {
    if (!list || !item) return;
    const char *target = (const char*)&item->data[0];
    for (int64_t i = 0; i < list->count; i++) {
        if (strcmp((const char*)&list->items[i]->data[0], target) == 0) {
            for (int64_t j = i; j < list->count - 1; j++) {
                list->items[j] = list->items[j + 1];
            }
            list->count--;
            return;
        }
    }
}

int64_t rv_list_has(RvList *list, RvString *item) {
    if (!list || !item) return 0;
    const char *target = (const char*)&item->data[0];
    for (int64_t i = 0; i < list->count; i++) {
        if (strcmp((const char*)&list->items[i]->data[0], target) == 0) {
            return 1;
        }
    }
    return 0;
}

int64_t rv_list_count(RvList *list) {
    return list ? list->count : 0;
}

RvString* rv_list_get(RvList *list, int64_t idx) {
    if (!list || idx < 0 || idx >= list->count) return rv_cstr_to_str("");
    return list->items[idx];
}

// Benchmarking
int64_t rv_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (int64_t)(ts.tv_sec * 1000 + ts.tv_nsec / 1000000);
}

void rv_measure_report(int64_t start_ms) {
    int64_t elapsed = rv_time_now_ms() - start_ms;
    printf("⏱️  Executed in %ld ms\n", elapsed);
}

// Pro Mode: Cycles benchmarking
void rv_measure_cycles_report(uint64_t cycles) {
    printf("⏱️  Executed in %llu CPU cycles\n", (unsigned long long)cycles);
}

// Pro Mode: Raw memory
int64_t rv_mem_alloc(int64_t size) {
    if (size <= 0) size = 8;
    void *ptr = malloc((size_t)size);
    if (ptr) memset(ptr, 0, (size_t)size);
    return (int64_t)(uintptr_t)ptr;
}

void rv_mem_free(int64_t ptr) {
    if (ptr) {
        free((void*)(uintptr_t)ptr);
    }
}

// Pro Mode: Threads
typedef struct {
    void *(*fn)(void*);
    void *arg;
} RvThreadArg;

static void* rv_thread_wrapper(void *data) {
    RvThreadArg *t = (RvThreadArg*)data;
    void *(*f)(void*) = t->fn;
    void *a = t->arg;
    free(t);
    f(a);
    return NULL;
}

int64_t rv_thread_spawn(void *(*func)(void *), void *arg) {
    pthread_t th;
    RvThreadArg *t = (RvThreadArg*)malloc(sizeof(RvThreadArg));
    t->fn = func;
    t->arg = arg;
    if (pthread_create(&th, NULL, rv_thread_wrapper, t) != 0) {
        free(t);
        return -1;
    }
    return (int64_t)th;
}

void rv_thread_join(int64_t tid) {
    if (tid > 0) {
        pthread_join((pthread_t)tid, NULL);
    }
}

// Pro Mode: Math
int64_t rv_sqrt(int64_t x) {
    return (int64_t)sqrt((double)x);
}

int64_t rv_sin(int64_t x) {
    return (int64_t)sin((double)x);
}

int64_t rv_cos(int64_t x) {
    return (int64_t)cos((double)x);
}

int64_t rv_pow(int64_t base, int64_t exp) {
    return (int64_t)pow((double)base, (double)exp);
}

int64_t rv_abs(int64_t x) {
    return (int64_t)labs(x);
}

int64_t rv_getpid(void) {
    return (int64_t)getpid();
}

// Pro Mode: Sockets & Networking
int64_t rv_tcp_listen(int64_t port) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return -1;

    int opt = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons((uint16_t)port);

    if (bind(fd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -1;
    }

    if (listen(fd, 10) < 0) {
        close(fd);
        return -1;
    }

    return (int64_t)fd;
}

int64_t rv_tcp_accept(int64_t server_fd) {
    if (server_fd < 0) return -1;
    struct sockaddr_in client_addr;
    socklen_t client_len = sizeof(client_addr);
    int client_fd = accept((int)server_fd, (struct sockaddr*)&client_addr, &client_len);
    return (int64_t)client_fd;
}

int64_t rv_tcp_connect(RvString *host_str, int64_t port) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return -1;

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons((uint16_t)port);

    const char *host = host_str ? (const char*)&host_str->data[0] : "127.0.0.1";
    if (inet_pton(AF_INET, host, &addr.sin_addr) <= 0) {
        close(fd);
        return -1;
    }

    if (connect(fd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -1;
    }

    return (int64_t)fd;
}

int64_t rv_tcp_send(int64_t fd, RvString *msg) {
    if (fd < 0 || !msg) return -1;
    return (int64_t)send((int)fd, &msg->data[0], msg->len, 0);
}

RvString* rv_tcp_recv(int64_t fd, int64_t max_bytes) {
    if (fd < 0 || max_bytes <= 0) return rv_cstr_to_str("");
    char *buf = (char*)malloc(max_bytes + 1);
    ssize_t n = recv((int)fd, buf, max_bytes, 0);
    if (n <= 0) {
        free(buf);
        return rv_cstr_to_str("");
    }
    buf[n] = '\0';
    RvString *res = (RvString*)malloc(sizeof(RvString) + n + 1);
    res->len = (uint64_t)n;
    memcpy(&res->data[0], buf, n + 1);
    free(buf);
    return res;
}

void rv_tcp_close(int64_t fd) {
    if (fd >= 0) {
        close((int)fd);
    }
}
