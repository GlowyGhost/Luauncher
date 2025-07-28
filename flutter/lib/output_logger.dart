import 'package:flutter/material.dart';

class Logger extends ChangeNotifier {
    final List<String> _logs = [];

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
