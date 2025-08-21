import 'package:flutter/material.dart';

import 'settings_screen.dart';

class OutputScreen extends StatefulWidget {
  const OutputScreen({super.key});

  @override
  State<OutputScreen> createState() => _OutputScreenState();
}

class _OutputScreenState extends State<OutputScreen> {
	void showBar(String text) {
		ScaffoldMessenger.of(context).showSnackBar(
			SnackBar(content: Text(text)),
		);
	}

	Future<void> save() async {
		List<String> logs = logger._logs;
		String log = logs.join("\n");

		String res = await tauriInvoke('save_log', {"log": log});

		if (res == "Success") {
			showBar("Saved File Successfully");
		} else if (res == "Cancelled") {
			showBar("Cancelled Save");
		} else {
			showBar("An Error Occurred. Please Try again Later.");
			if (settings.isDevMode) {
				logger.add("[output_screen.dart] Error saving file: $res");
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
								onPressed: logger.clear,
								icon: const Icon(Icons.clear),
								label: Text("Clear", style: TextStyle(fontSize: 18))
							),
							const SizedBox(width: 10),
							ElevatedButton.icon(
								onPressed: save,
								icon: const Icon(Icons.save),
								label: Text("Save", style: TextStyle(fontSize: 18))
							)
						],
					)
        ),

        Expanded(
          child: Scaffold(
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
          )
        )
      ],
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
