import 'package:js/js.dart';
import 'dart:js_util';
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:launcher/screens/settings_screen.dart';

import 'screens/output_screen.dart';

@JS('window.__TAURI__.core.invoke')
external dynamic _invoke(String cmd, [dynamic args]);

@JS('window.__TAURI__.event.listen')
external dynamic _listen(String eventName, Function callback);

Future<dynamic> tauriInvoke(String cmd, [Map<String, dynamic>? args]) async {
  if (settings.isDevMode) {
    logger.add("[tauri_invoke.dart] Invoking command $cmd");
  }
  
  try {
    final jsArgs = args != null ? jsify(args) : null;
    final promise = _invoke(cmd, jsArgs);
    final result = await promiseToFuture(promise);

    if (result is List) {
      return List<String>.from(result.map((e) => e.toString()));
    } else if (result is String) {
      return result;
    } else if (result == null) {
      return null;
    } else if (result is! String && hasProperty(result, 'dark')) {
      return {
        'dark': getProperty(result, 'dark'),
        'dev': getProperty(result, 'dev'),
        'close': getProperty(result, 'close'),
        'games': getProperty(result, 'games')
      };
    }

    return result;
  } catch (e) {
    logger.add("[Tauri Invoke Error] $e");
    rethrow;
  }
}

Future<Image> base64ToImage(String base64String) async {
	if (base64String.isEmpty) {
		throw Exception("Base64 icon data is empty");
	}

	try {
		final Uint8List bytes = base64Decode(base64String);
		return Image.memory(bytes);
	} catch (e) {
		throw Exception("Failed to decode base64 icon: $e");
	}
}

void createListeners() {
	if (settings.isDevMode) {
		logger.add("[tauri_invoke.dart] Creating Listeners");
	}

	_listen('log', allowInterop((dynamic event) {
		try {
			final payload = event['payload'];
			final log = payload['info'];

			logger.add("Hello");
			logger.add(log);
		} catch (e) {
			logger.add("Error processing event: $e");
		}
	}));
}
