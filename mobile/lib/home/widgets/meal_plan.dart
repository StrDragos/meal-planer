import 'package:flutter/material.dart';
import 'package:material_symbols_icons/symbols.dart';

class MealPlan extends StatelessWidget {
  const MealPlan({super.key});

  @override
  Widget build(BuildContext context) {
    return const Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      spacing: 32,
      children: [_MealPlanContent(), _QuickActions()],
    );
  }
}

class _MealPlanContent extends StatelessWidget {
  const _MealPlanContent();

  @override
  Widget build(BuildContext context) {
    return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
      Text("Today's Meal Plan", style: Theme.of(context).textTheme.headlineSmall),
      const _NoMealPlan(),
    ]);
  }
}

class _NoMealPlan extends StatelessWidget {
  const _NoMealPlan();

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          spacing: 16,
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Symbols.calendar_month,
              size: 46,
              color: Theme.of(context).colorScheme.primary,
            ),
            Text(
              "No Meal Plan Yet",
              style: Theme.of(context).textTheme.titleMedium,
            ),
            Text(
              "Plan your meals for the week to stay organized and eat healthier",
              style: Theme.of(context).textTheme.displayMedium,
              textAlign: TextAlign.center,
            ),
            FilledButton(
                onPressed: () {},
                child: const Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [Icon(Symbols.add_2_sharp), Text("Create Meal Plan")],
                ))
          ],
        ),
      ),
    );
  }
}

class _QuickActions extends StatelessWidget {
  const _QuickActions({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      spacing: 16,
      children: [
        QuickAction(title: "Browse Recipes", description: "Explore our collection", icon: Symbols.book_4),
        QuickAction(title: "Shopping List", description: "Plan your groceries", icon: Symbols.shopping_cart),
      ],
    );
  }
}

class QuickAction extends StatelessWidget {
  final IconData _icon;
  final String _title;
  final String _description;

  const QuickAction({title, description, icon, super.key})
      : _title = title,
        _description = description,
        _icon = icon;

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Icon(
              _icon,
              fill: 1,
              color: Theme.of(context).colorScheme.primary,
            ),
            Text(_title, style: Theme.of(context).textTheme.titleSmall),
            Text(_description, style: Theme.of(context).textTheme.bodySmall),
          ],
        ),
      ),
    );
  }
}
