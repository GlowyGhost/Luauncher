import 'package:flutter/material.dart';
import 'launcher_window.dart';

void main() {
  	runApp(const MyApp());
}

class MyApp extends StatelessWidget {
	const MyApp({super.key});

	@override
	Widget build(BuildContext context) {
		return MaterialApp(
			title: 'Game Launcher',
			debugShowCheckedModeBanner: false,
			theme: ThemeData(
				scaffoldBackgroundColor: const Color(0xFF262626),
				brightness: Brightness.dark,
				primaryColor: Color(0xFF262626),
				fontFamily: 'Segoe UI',
			),
			home: const LauncherWindow(),
		);
	}
}