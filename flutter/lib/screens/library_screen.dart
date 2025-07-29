import 'package:flutter/material.dart';
import 'dart:js' as js;

import '../tauri_invoke.dart';
import 'output_screen.dart';
import 'settings_screen.dart';

class LibraryScreen extends StatefulWidget {
	const LibraryScreen({super.key});

	@override
	State<LibraryScreen> createState() => _LibraryScreenState();
}

class _LibraryScreenState extends State<LibraryScreen> {
	List<String> _games = [];
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
		final games = await tauriInvoke('get_games');
		setState(() {
			_games = games;
			_loading = false;
		});

    if (settings.isDevMode) {
      logger.add("[library.dart] Got games $games");
    }
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
		ScaffoldMessenger.of(context).showSnackBar(
			const SnackBar(content: Text('Add Game pressed')),
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

										return ListTile(
											title: Text(game, style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black)),

											leading: const Icon(Icons.videogame_asset),

											/*
													Doesn't work. Check main.rs to see what I said about this        -- GlowyGhost 25/7/25
											 leading: FutureBuilder<Image>(
												future: tauriInvoke("get_icon", {"game_name": _games[index]})
													.then((base64String) => base64ToImage(base64String)),
												builder: (context, snapshot) {
													if (snapshot.connectionState == ConnectionState.waiting) {
														return const SizedBox(
															width: 24,
															height: 24,
															child: CircularProgressIndicator(strokeWidth: 2),
														);
													} else if (snapshot.hasError) {
														ScaffoldMessenger.of(context).showSnackBar(
															SnackBar(content: Text('${snapshot.data} for ${_games[index]}')),
														);

														return const Icon(Icons.error);
													} else if (snapshot.hasData) {
														return SizedBox(
															width: 32,
															height: 32,
															child: snapshot.data!,
														);
													} else {
														return const Icon(Icons.image_not_supported);
													}
												},
											), */

											tileColor: isSelected ? Colors.grey[800] : null,
											onTap: () => _launchGame(game),
										);
									},
								),
							),
				),
			],
		);
	}
}
