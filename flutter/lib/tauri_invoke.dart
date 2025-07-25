import 'package:js/js.dart';
import 'dart:js_util';
import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter/material.dart';

@JS('window.__TAURI__.core.invoke')
external dynamic _invoke(String cmd, [dynamic args]);

Future<dynamic> tauriInvoke(String cmd, [Map<String, dynamic>? args]) async {
	final promise = _invoke(cmd, args);
	final result = await promiseToFuture(promise);
		
	// Convert from JS array to Dart list if needed
	if (result is List) {
		return List<String>.from(result.map((e) => e.toString()));
	} else if (result is String) {
		return result;
	}

	throw Exception('Unexpected result from Tauri: $result');
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
