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

Future<dynamic> tauriInvoke(String cmd, [Map<String, dynamic>? args]) async {
  if (settings.isDevMode && cmd != "get_logs") {
    logger.add("[tauri_invoke.dart] Invoking command $cmd");
  }
  
  try {
    final jsArgs = args != null ? jsify(args) : null;
    final promise = _invoke(cmd, jsArgs);
    final result = await promiseToFuture(promise);

    if (result is List) {
      if (cmd == "get_logs") {
        return result.map((e) {
          if (e is String) return e;
          if (hasProperty(e, 'message') && hasProperty(e, 'level')) {
            return {
              'message': getProperty(e, 'message'),
              'level': getProperty(e, 'level'),
              'dev_mode': getProperty(e, 'dev_mode')
            };
          }
          return e.toString();
        }).toList();
      } else if (cmd == "get_games") {
        return List<String>.from(result.map((e) => e.toString()));
      }
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
