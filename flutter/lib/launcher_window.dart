import 'package:flutter/material.dart';
import 'screens/library_screen.dart';
import 'screens/settings_screen.dart';
import 'screens/output_screen.dart';

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
			backgroundColor: const Color(0xFF262626),
			body: Row(
				children: [
					Padding(
						padding: const EdgeInsets.only(top: 5, bottom: 5, left: 5),
						child: Container(
							width: screenWidth / 3,
							decoration: BoxDecoration(
								color: const Color(0xFF1F1F1F),
								borderRadius: BorderRadius.circular(12),
							),
							child: Column(
								crossAxisAlignment: CrossAxisAlignment.stretch,
								children: [
								const SizedBox(height: 60),
									_buildNavButton('Library', LauncherPage.library),
									_buildNavButton('Settings', LauncherPage.settings),
									_buildNavButton('Output', LauncherPage.output),
								],
							),
						),
					),

					Expanded(
						child: Container(
							color: const Color(0xFF262626),
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
				foregroundColor: isSelected ? Colors.white : Colors.grey,
				padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 20),
				alignment: Alignment.centerLeft,
			),
			onPressed: () => setState(() => _currentPage = page),
			child: Text(label, style: const TextStyle(fontSize: 18)),
		);
	}
}
