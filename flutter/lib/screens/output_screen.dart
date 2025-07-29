import 'package:flutter/material.dart';

import 'settings_screen.dart';

class OutputScreen extends StatefulWidget {
  const OutputScreen({super.key});

  @override
  State<OutputScreen> createState() => _OutputScreenState();
}

class _OutputScreenState extends State<OutputScreen> {
  	@override
    Widget build(BuildContext context) {
		return Scaffold(
			body: AnimatedBuilder(
				animation: logger,
				builder: (context, _) {
					return ListView.builder(
						padding: const EdgeInsets.all(16),
						itemCount: logger.logs.length,
						itemBuilder: (context, index) {
							final log = logger.logs[index];
							return Text(log, style: TextStyle(fontSize: 18, color: settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black));
						},
					);
				},
			),
		);
  }
}

class Logger extends ChangeNotifier {
    final List<String> _logs = [];
    final bool devMode = false;

    List<String> get logs => List.unmodifiable(_logs);

    void add(String message) {
        _logs.add(message);
        notifyListeners();
    }

    void clear() {
        _logs.clear();
        notifyListeners();
    }
}

final logger = Logger();
