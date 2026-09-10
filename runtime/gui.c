#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>

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
