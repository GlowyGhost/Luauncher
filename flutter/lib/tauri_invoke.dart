import 'package:js/js.dart';
import 'dart:js_util';
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter/material.dart';

import 'output_logger.dart';

@JS('window.__TAURI__.core.invoke')
external dynamic _invoke(String cmd, [dynamic args]);

Future<dynamic> tauriInvoke(String cmd, [Map<String, dynamic>? args]) async {
  logger.add("[Tauri Invoke] Invoking command $cmd");
  logger.add("[Tauri Invoke Args] cmd=$cmd args=${jsonEncode(args)}");

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
    }

    return result;
  } catch (e, st) {
    logger.add("[Tauri Invoke Error] $e");
    logger.add("[STACKTRACE] $st");
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
