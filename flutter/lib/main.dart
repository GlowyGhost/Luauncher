import 'package:flutter/material.dart';
import 'screens/output_screen.dart';
import 'launcher_window.dart';
import 'screens/settings_screen.dart';

void main() async {
  runApp(const App());
}

class App extends StatefulWidget {
  const App({super.key});

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> {
  bool _isLoaded = false;

  @override
  void initState() {
    super.initState();
    _loadSettings();
  }

  Future<void> _loadSettings() async {
    try {
      await settings.loadSettings();
    } catch (e) {
      logger.add("Error loading settings: $e");
    }

    setState(() {
      _isLoaded = true;
    });

    if (settings.isDevMode) {
      logger.add("[main.dart] Successfully opened");
    }
  }

  @override
  Widget build(BuildContext context) {
    if (!_isLoaded) {
      return const MaterialApp(
        home: Scaffold(
          body: Center(child: CircularProgressIndicator()),
        ),
      );
    }

    return MaterialApp(
        title: 'Game Launcher',
        debugShowCheckedModeBanner: false,
        theme: ThemeData(
            scaffoldBackgroundColor: settings.oldDarkMode
                ? const Color(0xFF262626)
                : const Color(0xFFCCCCCC),
            primaryColor: settings.oldDarkMode
                ? const Color(0xFF262626)
                : const Color(0xFFCCCCCC),
        ),
        home: const LauncherWindow(),
    );
  }
}

