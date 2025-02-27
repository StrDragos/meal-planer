import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:material_symbols_icons/symbols.dart';

final class BaseScreen extends StatelessWidget {
  final Widget _body;

  const BaseScreen({super.key, required Widget body}) : _body = body;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      bottomNavigationBar: NavigationBar(
        onDestinationSelected: (index) {
          switch (index) {
            case 1: context.go("/"); break;
            case 2: context.go("/receipts"); break;
            case 3: context.go("/add"); break;
            case 4: context.go("/profile"); break;
          }
        },
        destinations: const [
          NavigationDestination(icon: Icon(Symbols.home, fill: 1), label: "Home"),
          NavigationDestination(icon: Icon(Symbols.book_sharp, fill: 1), label: "Recipes"),
          NavigationDestination(icon: Icon(Symbols.calendar_month_rounded, fill: 1), label: "Meal Plan"),
          NavigationDestination(icon: Icon(Symbols.add_2, fill: 1), label: "Add"),
          NavigationDestination(icon: Icon(Symbols.person_2_rounded, fill: 1), label: "Profile"),
        ],
      ),
      appBar: AppBar(
        title: Align(
          alignment: Alignment.centerLeft,
          child: Text("CookingPal", style: Theme.of(context).textTheme.titleLarge),
        ),
        actions: [
          IconButton(onPressed: () {}, icon: const Icon(Symbols.search)),
          IconButton(
              onPressed: () {},
              icon: Badge.count(count: 3, child: const Icon(Symbols.notifications_none_rounded, fill: 1)))
        ],
      ),
      body: _body,
    );
  }
}
