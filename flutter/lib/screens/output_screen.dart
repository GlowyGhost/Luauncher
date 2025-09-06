import 'package:flutter/material.dart';

import '../tauri_invoke.dart';
import 'settings_screen.dart';

class OutputScreen extends StatefulWidget {
  const OutputScreen({super.key});

  @override
  State<OutputScreen> createState() => _OutputScreenState();
}

class _OutputScreenState extends State<OutputScreen> {
  @override
  void initState() {
    super.initState();
    _startLogPolling();
  }

  void _startLogPolling() {
    Future.doWhile(() async {
      await logger.getRustLogs();
      await Future.delayed(const Duration(seconds: 1));
      return mounted;
    });
  }

	void showBar(String text) {
		ScaffoldMessenger.of(context).showSnackBar(
			SnackBar(content: Text(text)),
		);
	}

	Future<void> save() async {
		List<LogEntry> logs = logger._logs;
		String log = logs.map((e) => "[${e.level}] ${e.message}").join("\n");

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

                    final color = switch (log.level) {
                        LogLevel.Info => settings.oldDarkMode ? Color(0xFFFFFFFF) : Colors.black,
                        LogLevel.Warning => settings.oldDarkMode ? Color(0xFFFF9830) : Color(0xFFD96D00),
                        LogLevel.Error => settings.oldDarkMode ? Color(0xFFFF2626) : Color(0xFFE00000),
                    };

                    return Text(log.message, style: TextStyle(fontSize: 18, color: color));
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

enum LogLevel { Info, Warning, Error }

class LogEntry {
  final String message;
  final LogLevel level;

  LogEntry(this.message, this.level);
}

class Logger extends ChangeNotifier {
    final List<LogEntry> _logs = [];
    final bool devMode = false;

    List<LogEntry> get logs => List.unmodifiable(_logs);

    void add(String message, {LogLevel level = LogLevel.Info}) {
      _logs.add(LogEntry(message, level));
      notifyListeners();
    }

    Future<void> getRustLogs() async {
      final res = await tauriInvoke('get_logs');

      if (res is List) {
        for (var log in res) {
          if (log is Map<String, dynamic>) {
            if (log['dev_mode'] == true && !settings.isDevMode) {
              continue;
            }

            LogLevel level = switch (log['level'].toString()) {
              'Info' => LogLevel.Info,
              'Warning' => LogLevel.Warning,
              'Error' => LogLevel.Error,
              _ => LogLevel.Info,
            };

            add("${log['message']}", level: level);
          }
        }
      }
    }

    void clear() {
        _logs.clear();
        notifyListeners();
    }
}

final logger = Logger();
