import 'package:flutter/material.dart';

class LibraryScreen extends StatefulWidget {
	const LibraryScreen({super.key});

	@override
	State<LibraryScreen> createState() => _LibraryScreenState();
}

class _LibraryScreenState extends State<LibraryScreen> {
	List<String> _games = [];
	bool _loading = true;
	String? _selectedGame;

	@override
	void initState() {
		super.initState();
		_loadGames();
	}

	Future<void> _loadGames() async {
		setState(() => _loading = true);
		final games = ["Portal", "Portal 2", "Half-Life", "Crab Game"]; // Temporary, TODO: Get all TRUE files through rust (probobly, planned to do that currently)
		setState(() {
			_games = games;
			_loading = false;
		});
	}

	void _launchGame(game) {
		ScaffoldMessenger.of(context).showSnackBar(
			SnackBar(content: Text('Launching $game')),
		);
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
								label: const Text("Add"),
							),
							const SizedBox(width: 10),
							ElevatedButton.icon(
								onPressed: _loadGames,
								icon: const Icon(Icons.refresh),
								label: const Text("Refresh"),
							),
						],
					),
				),

				Expanded(
					child: _loading
						? const Center(child: CircularProgressIndicator())
						: _games.isEmpty
							? const Center(child: Text("No games found."))
							: Material(
								color: Colors.transparent,
								child: ListView.builder(
									itemCount: _games.length,
									itemBuilder: (context, index) {
										final game = _games[index];
										final isSelected = _selectedGame == game;

										return ListTile(
											title: Text(_games[index]),
											leading: const Icon(Icons.videogame_asset),
											tileColor: isSelected ? Colors.grey[800] : null,
											onTap: () => _launchGame(_games[index]),
										);
									},
								),
							),
				),
			],
		);
	}
}