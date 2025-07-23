#include "flutter_window.h"

#include <dwmapi.h>
#include <windowsx.h>

#pragma comment(lib, "dwmapi.lib")

#include "flutter/generated_plugin_registrant.h"

FlutterWindow::FlutterWindow(const flutter::DartProject& project)
    : project_(project) {}

FlutterWindow::~FlutterWindow() {}

bool FlutterWindow::OnCreate() {
  if (!Win32Window::OnCreate()) {
    return false;
  }

  RECT frame = GetClientArea();

  // Create Flutter view controller with client size
  flutter_controller_ = std::make_unique<flutter::FlutterViewController>(
      frame.right - frame.left, frame.bottom - frame.top, project_);

  if (!flutter_controller_->engine() || !flutter_controller_->view()) {
    return false;
  }

  RegisterPlugins(flutter_controller_->engine());

  SetChildContent(flutter_controller_->view()->GetNativeWindow());

  // Set the window style to normal overlapped window with title bar and borders
  LONG style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
  SetWindowLong(GetHandle(), GWL_STYLE, style);

  // Call our custom method to set the native title bar color
  SetTitleBarColor(GetHandle(), RGB(38, 38, 38));  // Dark gray color (adjust as needed)

  flutter_controller_->engine()->SetNextFrameCallback([&]() {
    this->Show();
  });

  flutter_controller_->ForceRedraw();

  return true;
}

void FlutterWindow::SetTitleBarColor(HWND hwnd, COLORREF color) {
  // DWMWA_CAPTION_COLOR = 35 (Windows 10 1809+)
  DwmSetWindowAttribute(hwnd, 35, &color, sizeof(color));

  // Set title bar text color (white here)
  COLORREF textColor = RGB(255, 255, 255);
  DwmSetWindowAttribute(hwnd, 36, &textColor, sizeof(textColor));
}

void FlutterWindow::OnDestroy() {
  if (flutter_controller_) {
    flutter_controller_ = nullptr;
  }
  Win32Window::OnDestroy();
}

LRESULT FlutterWindow::MessageHandler(HWND hwnd, UINT const message,
                                      WPARAM const wparam,
                                      LPARAM const lparam) noexcept {
  switch (message) {
    case WM_NCHITTEST: {
      POINT cursor_pos = {GET_X_LPARAM(lparam), GET_Y_LPARAM(lparam)};
      ScreenToClient(hwnd, &cursor_pos);

      // Make top 40 pixels draggable
      if (cursor_pos.y < 40) {
        return HTCAPTION;
      }
      break;
    }
  }
  return Win32Window::MessageHandler(hwnd, message, wparam, lparam);
}