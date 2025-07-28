import 'package:flutter/material.dart';
import '../output_logger.dart';

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
							return Text(
								log,
								style: const TextStyle(fontSize: 14, color: Colors.white),
							);
						},
					);
				},
			),
		);
  }
}
