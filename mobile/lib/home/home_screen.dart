import 'package:flutter/material.dart';
import 'package:mobile/common/widgets/base_screen.dart';
import 'package:mobile/home/widgets/carousel.dart';
import 'package:mobile/home/widgets/meal_plan.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return BaseScreen(
        body: Padding(
          padding: const EdgeInsets.all(16.0),
          child: ListView(
            children: const [
              HeaderText(),
              SizedBox(height: 32,),
              MealPlan(),
              SizedBox(height: 32,),
              DishCarousel()
            ],
          ),
        ));
  }
}


class HeaderText extends StatelessWidget {
  const HeaderText({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text("Welcome back, Sarah!", style: Theme.of(context).textTheme.headlineMedium,),
        Text("Ready to cook something delicious?", style: Theme.of(context).textTheme.bodyMedium)
      ],
    );
  }
}
