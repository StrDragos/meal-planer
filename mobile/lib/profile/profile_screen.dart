import 'package:flutter/material.dart';

class ProfileScreen extends StatelessWidget {
  const ProfileScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Text("Hello profile", style: Theme.of(context).textTheme.titleMedium);
  }
}
