import 'package:flutter/material.dart';
import 'dart:js' as js;

import '../tauri_invoke.dart';
import 'output_screen.dart';
import 'settings_screen.dart';

class GameInfo {
	final String name;
	final Image? icon;

	GameInfo(this.name, this.icon);
}

class LibraryScreen extends StatefulWidget {
	const LibraryScreen({super.key});

	@override
	State<LibraryScreen> createState() => _LibraryScreenState();
}

class _LibraryScreenState extends State<LibraryScreen> {
	List<GameInfo> _games = [];
	bool _loading = true;
	String? _selectedGame;

	bool get isTauriAvailable {
		final tauri = js.context['__TAURI__'];
		return tauri != null && tauri is js.JsObject;
	}

	@override
	Future<void> initState() async {
		super.initState();
		_init();
	}

	
	Future<void> _init() async {
		await _loadGames();

		if (mounted && isTauriAvailable == false) {
			ScaffoldMessenger.of(context).showSnackBar(
				SnackBar(content: Text('Unable to connect with backend.')),
			);
		}
	}

	Future<void> _loadGames() async {
		setState(() => _loading = true);
		final gameNames = await tauriInvoke('get_games') as List<String>;

		List<GameInfo> loadedGames = [];

		for (final name in gameNames) {
			try {
				final exePath = await tauriInvoke("get_game_path", {"gameName": name});
				final base64Icon = await tauriInvoke("get_icon", {"exePath": exePath, "name": name});

				if (base64Icon == "" || base64Icon == null || base64Icon.isEmpty) {
					loadedGames.add(GameInfo(name, null));
				} else {
					final icon = await base64ToImage(base64Icon);

					loadedGames.add(GameInfo(name, icon));
				}
			} catch (e) {
				loadedGames.add(GameInfo(name, null));

				if (settings.isDevMode) {
					logger.add("Error loading icon for $name: $e");
				}
			}
		}

		setState(() {
			_games = loadedGames;
			_loading = false;
		});
	}

	Future<void> _launchGame(String game) async {
		if (settings.isDevMode) {
			logger.add("[library.dart] Opening game $game");
		}

		if (settings.closeAfterOpen) {
			await tauriInvoke("hide_app");
		}

		ScaffoldMessenger.of(context).showSnackBar(
			SnackBar(content: Text('Launching $game')),
		);

		await tauriInvoke("run_game", {"gameName": game});
	}

	void _onAddGame() {
		String path = "";
		String name = "";
		String code = "";

		showDialog(
            context: context,
            builder: (context) => AlertDialog(
                title: Text('Add Game'),
                content: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                        TextField(
                            decoration: InputDecoration(labelText: 'Name'),
                            onChanged: (value) => name = value,
                        ),
                        TextField(
                            decoration: InputDecoration(labelText: 'Path to executable'),
                            onChanged: (value) => path = value,
                        ),
                        TextField(
                            decoration: InputDecoration(labelText: "Path to script"),
                            onChanged: (value) => code = value,
                        ),
                    ],
                ),
                actions: [
					TextButton(
						onPressed: () => Navigator.pop(context),
						child: Text('Cancel'),
					),
					TextButton(
						onPressed: () async {
							if (settings.isDevMode) {
								logger.add("[library.dart] name: $name");
								logger.add("[library.dart] path: $path");
								logger.add("[library.dart] code: $code");
							}

							Navigator.pop(context);

							for (final game in _games) {
								if (game.name == name) {
									ScaffoldMessenger.of(context).showSnackBar(
										SnackBar(content: Text('Game $name already exists.')),
									);

									return;
								}
							}

							await tauriInvoke("make_plugin", {"name": name, "path": path, "code": code});

							_loadGames();
						},
						child: Text('Add'),
					),
                ],
            ),
        );
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
								onPressed: _onAddGame,
								icon: const Icon(Icons.add),
								label: Text("Add"),
							),
							const SizedBox(width: 10),
							ElevatedButton.icon(
								onPressed: _loadGames,
								icon: const Icon(Icons.refresh),
								label: Text("Refresh"),
							),
						],
					),
				),

				Expanded(
					child: _loading
						? const Center(child: CircularProgressIndicator())
						: _games.isEmpty
							? Center(child: Text("No games found.", style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)))
							: Material(
								color: Colors.transparent,
								child: ListView.builder(
									itemCount: _games.length,
									itemBuilder: (context, index) {
										final game = _games[index];
										final isSelected = _selectedGame == game;

										String name = game.name;
										String path = "";

										return ListTile(
											title: Text(game.name, style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)),
											leading: game.icon ?? const Icon(Icons.videogame_asset),
											tileColor: isSelected ? Colors.grey[800] : null,
											onTap: () => _launchGame(game.name),
											
											trailing: PopupMenuButton<String>(
												icon: Icon(Icons.more_vert, color: settings.oldDarkMode ? Colors.white : Colors.black),
												onSelected: (value) async {
													if (value == 'edit') {
														String defaultPath = await tauriInvoke("get_game_path", {"gameName": game.name});
														TextEditingController pathController = TextEditingController(text: defaultPath);
														TextEditingController nameController = TextEditingController(text: game.name);

														showDialog(
															context: context,
															builder: (context) => AlertDialog(
																title: Text(game.name),
																content: Column(
																	mainAxisSize: MainAxisSize.min,
																	children: [
																		TextField(
																			controller: nameController,
																			decoration: InputDecoration(labelText: 'Name'),
																			onChanged: (value) => name = value,
																		),
																		TextField(
																			controller: pathController,
																			decoration: InputDecoration(labelText: 'Path to executable'),
																			onChanged: (value) => path = value,
																		),
																	],
																),
																actions: [
																	TextButton(
																		onPressed: () => Navigator.pop(context),
																		child: Text('Cancel'),
																	),
																	TextButton(
																		onPressed: () async {
																			Navigator.pop(context);
																			String res = await tauriInvoke("save_game", {
																				"path": path,
																				"name": name,
																				"oldn": game.name
																			});

																			if (res == "Err") {
																				ScaffoldMessenger.of(context).showSnackBar(
																					SnackBar(content: Text("Error Saving: Game not found in settings.")),
																				);
																			}

																			_loadGames();
																		},
																		child: Text('Save'),
																	),
																],
															),
														);
														} else if (value == 'delete') {
															await tauriInvoke("delete_game", {"name": game.name});
															_loadGames();
														} else if (value == 'create') {
                              String res = await tauriInvoke("create_shortcut", {"name": game.name});

                              if (res == "Linux") {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  SnackBar(content: Text("Shortcut creation is only supported on Windows and macOS.")),
                                );
                              } else {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  SnackBar(content: Text("Shortcut created at: $res")),
                                );
                              }
                            }
													}, 
													itemBuilder: (context) => [
														PopupMenuItem(
															value: 'edit',
															child: Text('Edit'),
														),
														PopupMenuItem(
															value: 'delete',
															child: Text('Delete'),
														),
                            PopupMenuItem(
															value: 'create',
															child: Text('Create Shortcut'),
														),
													],
												),
										);
									},
								),
							),
				),
			],
		);
	}
}
