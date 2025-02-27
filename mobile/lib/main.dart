import 'package:flutter/material.dart';
import 'package:mobile/router_config.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  runApp(const ProviderScope(
    child: CookingPal(),
  ));
}

final themeData = ThemeData(
  colorScheme: ColorScheme.fromSeed(
      seedColor: Colors.white, // Primary green
      brightness: Brightness.light, // Or Brightness.dark for dark mode
      onSurface: const Color(0xFFF9FAFB),
      primary: const Color(0xFF059669)),
  textTheme: const TextTheme(
    titleSmall: TextStyle(fontSize: 16, color: Color(0xFF1F2937)),
    titleMedium: TextStyle(fontSize: 18, color: Color(0xFF1F2937), fontWeight: FontWeight.w600),
    titleLarge: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Color(0xFF059669)),
    headlineMedium: TextStyle(color: Color(0xFF1F2937), fontSize: 24, fontWeight: FontWeight.bold),
    headlineSmall: TextStyle(color: Color(0xFF1F2937), fontSize: 18, fontWeight: FontWeight.w600),
    bodyLarge: TextStyle(color: Color(0xFF1F2937), fontWeight: FontWeight.w600),
    bodyMedium: TextStyle(color: Color(0xFF6B7280), fontSize: 16, fontWeight: FontWeight.normal),
    bodySmall: TextStyle(color: Color(0xFF6B7280), fontSize: 12, fontWeight: FontWeight.normal),
    displayMedium: TextStyle(color: Color(0xFF6B7280), fontSize: 14),
    labelMedium: TextStyle(color: Color(0xFF757575), fontSize: 14),
  ),
  appBarTheme: const AppBarTheme(
    backgroundColor: Colors.white, // White background
  ),
  cardTheme: const CardTheme(
    color: Colors.white,
    shadowColor: Color(0xFFE5E7EB),
  ),
  navigationBarTheme: const NavigationBarThemeData().copyWith(
      backgroundColor: Colors.white,
      iconTheme: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected) ||
            states.contains(WidgetState.pressed) ||
            states.contains(WidgetState.focused)) {
          return const IconThemeData(color: Color(0xFF059669));
        } else {
          return const IconThemeData(color: Color(0xFF9CA3AF));
        }
      }),
      labelBehavior: NavigationDestinationLabelBehavior.alwaysShow,
      indicatorColor: Colors.transparent,
      labelTextStyle: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected) ||
            states.contains(WidgetState.pressed) ||
            states.contains(WidgetState.focused)) {
          return const TextStyle(color: Color(0xFF059669), fontSize: 12);
        } else {
          return const TextStyle(color: Color(0xFF9CA3AF), fontSize: 12);
        }
      })),
  useMaterial3: true,
);

class CookingPal extends StatelessWidget {
  const CookingPal({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      theme: themeData,
      routerConfig: routes,
    );
  }
}
