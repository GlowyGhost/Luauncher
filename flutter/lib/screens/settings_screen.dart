import 'package:flutter/material.dart';
import '../tauri_invoke.dart';
import 'output_screen.dart';

class SettingsScreen extends StatefulWidget {
	const SettingsScreen({super.key});

	@override
	State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
	@override
	void initState() {
		super.initState();
		_init();
	}

	
	Future<void> _init() async {
		await settings.loadSettings();
	}

	Future<void> _saveSettings() async {
		final res = await settings.saveSettings();

		if (res == "Saved Settings" && mounted) {
			ScaffoldMessenger.of(context).showSnackBar(
				SnackBar(content: Text('Saved Settings Successfully!')),
			);

            if (settings.isDevMode) {
                logger.add("[settings.dart] Saved Setings");
            }
		}
	}

	@override
	Widget build(BuildContext context) {
		return Column(
			children: [
				Padding(
					padding: const EdgeInsets.fromLTRB(10, 10, 10, 5),
					child: Row(
						children: [
							ElevatedButton.icon(
								onPressed: _saveSettings,
								icon: const Icon(Icons.save),
								label: Text("Save", style: TextStyle(fontSize: 18))
							)
						],
					)
				),

				Expanded(
					child: Scaffold(
						body: ListView(
							padding: const EdgeInsets.all(16),
							children: [
								const SizedBox(height: 20),

								SwitchListTile(
									title: Text("Dark Mode", style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)),
									value: settings.isDarkMode,
									onChanged: (value) {
										setState(() => settings.isDarkMode = value);
									},
								),

								const SizedBox(height: 20),

								SwitchListTile(
									title: Text("Dev Mode", style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)),
									value: settings.isDevMode,
									onChanged: (value) {
										setState(() => settings.isDevMode = value);
									},
								),

								const SizedBox(height: 20),

								SwitchListTile(
									title: Text("Close After Opening", style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)),
									value: settings.closeAfterOpen,
									onChanged: (value) {
										setState(() => settings.closeAfterOpen = value);
									},
								),

								/* const SizedBox(height: 10),

								// Textbox
								TextField(
									decoration: const InputDecoration(
										labelText: 'Username',
										border: OutlineInputBorder(),
									),
									onChanged: (value) {
										setState(() => _username = value);
									},
								),

								const SizedBox(height: 20),

								// Dropdown
								DropdownButtonFormField<String>(
									decoration: const InputDecoration(
										labelText: 'Language',
										border: OutlineInputBorder(),
									),
									value: _selectedLanguage,
									onChanged: (value) {
										if (value != null) {
											setState(() => _selectedLanguage = value);
										}
									},
									items: _languages.map((lang) {
										return DropdownMenuItem(value: lang, child: Text(lang));
									}).toList(),
								),

								const SizedBox(height: 20),

								// Slider
								Column(
									crossAxisAlignment: CrossAxisAlignment.start,
									children: [
										const Text("Volume"),
										Slider(
											value: _volume,
											min: 0,
											max: 1,
											divisions: 10,
											label: (_volume * 100).toInt().toString(),
											onChanged: (value) {
												setState(() => _volume = value);
											},
										),
									],
								), */
							],
						),
					)
				)
			],
		);
	}
}

class Settings extends ChangeNotifier {
	bool oldDarkMode = true;
	bool isDarkMode = true;
	bool isDevMode = false;
	bool closeAfterOpen = true;
	Map<String, String> gamePaths = {};

	Future<void> loadSettings() async {
		final settings = await tauriInvoke('get_settings');
		
		isDarkMode = settings["dark"];
		isDevMode = settings["dev"];
		closeAfterOpen = settings["close"];

		gamePaths = settings["games"];

		oldDarkMode = settings["dark"];
	}

	Future<String> saveSettings() async {
		String res = await tauriInvoke('save_settings', {"dark": isDarkMode, "dev": isDevMode, "close": closeAfterOpen, "games": gamePaths});

        if (oldDarkMode != isDarkMode) {
            if (settings.isDevMode) {
                logger.add("[settings.dart] Restarting app");
            }

            await tauriInvoke('restart_app');
        }

        return res;
	}
}

final settings = Settings();
