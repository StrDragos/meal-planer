import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:mobile/common/model/summary_dish.dart';

class SummaryDishCard extends StatelessWidget {
  final SummaryDish _dish;

  const SummaryDishCard({dish, super.key}) : _dish = dish;

  @override
  Widget build(BuildContext context) {
    Uint8List bytes = base64Decode(_dish.base64Image.split(",").last);
    return ColoredBox(
      color: Theme.of(context).cardTheme.color ?? Colors.white,
      child: Column(
        children: [
          Image.memory(
            bytes,
            width: 192,
            height: 128,
            fit: BoxFit.cover,
          ),
          Padding(
            padding: const EdgeInsets.only(left: 8, right: 8),
            child: Column(
              spacing: 2,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  _dish.name,
                  style: const TextStyle(fontSize: 14, color: Colors.black),
                ),
                Row(
                  children: [
                    Icon(Symbols.timer, color: Theme.of(context).colorScheme.surfaceTint, size: 16),
                    Text(
                      "${_dish.preparationMinutes} mins",
                      style: Theme.of(context).textTheme.bodySmall,
                    )
                  ],
                )
              ],
            ),
          )
        ],
      ),
    );
  }
}
