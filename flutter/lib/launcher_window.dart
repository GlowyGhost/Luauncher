import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'screens/library_screen.dart';
import 'screens/settings_screen.dart';
import 'screens/output_screen.dart';
import 'tauri_invoke.dart';

enum LauncherPage { library, settings, output }

class LauncherWindow extends StatefulWidget {
	const LauncherWindow({super.key});

	@override
	State<LauncherWindow> createState() => _LauncherWindowState();
}

class _LauncherWindowState extends State<LauncherWindow> {
	LauncherPage _currentPage = LauncherPage.library;

	@override
	Widget build(BuildContext context) {
		final screenWidth = MediaQuery.of(context).size.width;

		return Scaffold(
			backgroundColor: settings.oldDarkMode
                  ? Color(0xFF262626)
                  : Color(0xCCCCCCCC),
			body: Row(
				children: [
					Padding(
						padding: const EdgeInsets.only(top: 5, bottom: 5, left: 5),
						child: Container(
							width: screenWidth / 3,
							decoration: BoxDecoration(
								color: settings.oldDarkMode
                  ? Color(0xFF1F1F1F)
                  : Color(0xFFA1A1A1),
								borderRadius: BorderRadius.circular(12),
							),
							child: Scaffold(
                backgroundColor: Colors.transparent,
                body: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    const SizedBox(height: 60),
                    _buildNavButton('Library', LauncherPage.library),
                    _buildNavButton('Settings', LauncherPage.settings),
                    _buildNavButton('Output', LauncherPage.output),
                  ],
                ),

                bottomNavigationBar: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      "Version: ${settings.version}",
                      style: TextStyle(
                        fontSize: 16,
                        color: settings.oldDarkMode
                          ? Color(0xFFFFFFFF)
                          : Colors.black
                      ),
                    ),
                    Row(
                      mainAxisSize: MainAxisSize.min,
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        IconButton(
                          icon: Icon(FontAwesomeIcons.github, color: settings.oldDarkMode
                            ? Color(0xFFFFFFFF)
                            : Colors.black
                          ),
                          onPressed: () async {
                            await tauriInvoke("open_link", {"url": "https://github.com/GlowyGhost/Luauncher"});
                          },
                        )
                      ],
                    )
                  ],
                ),
              )
						),
					),

					Expanded(
						child: Container(
							color: settings.oldDarkMode
                  ? Color(0xFF262626)
                  : Color(0xFFCCCCCC),
							child: switch (_currentPage) {
								LauncherPage.library => const LibraryScreen(),
								LauncherPage.settings => const SettingsScreen(),
								LauncherPage.output => const OutputScreen(),
							},
						),
					),
				],
			),
		);
	}

	Widget _buildNavButton(String label, LauncherPage page) {
		final isSelected = _currentPage == page;
		return TextButton(
			style: TextButton.styleFrom(
				foregroundColor: isSelected ? 
          settings.oldDarkMode
            ? Color(0xFFFFFFFF)
            : Color(0xFF545454)
          : settings.oldDarkMode
            ? Color(0xFFB3B3B3)
            : Colors.black,
				padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 20),
				alignment: Alignment.centerLeft,
			),
			onPressed: () => setState(() => _currentPage = page),
			child: Text(label),
		);
	}
}
