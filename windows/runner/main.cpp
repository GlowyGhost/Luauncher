#include <flutter/dart_project.h>
#include <flutter/flutter_view_controller.h>
#include <windows.h>

#include "flutter_window.h"
#include "utils.h"
#include "bitsdojo_window_windows/bitsdojo_window_plugin.h"

int APIENTRY wWinMain(HINSTANCE instance, HINSTANCE prev, wchar_t* command_line, int show_command) {
  ::SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

  flutter::DartProject project(L"data");
  FlutterWindow window(project);

  Win32Window::Point origin(10, 10);
  Win32Window::Size size(900, 600);

  if (!window.Create(L"Game Launcher", origin, size)) {
      return EXIT_FAILURE;
  }

  window.Show();

  window.SetQuitOnClose(true);

  ::ShowWindow(GetConsoleWindow(), SW_HIDE); // optional: hides console window

  MSG msg;
  while (GetMessage(&msg, nullptr, 0, 0)) {
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }

  return EXIT_SUCCESS;
}