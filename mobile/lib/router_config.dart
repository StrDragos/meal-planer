import 'package:go_router/go_router.dart';
import 'package:mobile/profile/profile_screen.dart';

import 'home/home_screen.dart';

final routes = GoRouter(routes: [
  GoRoute(path: "/", builder: (context, state) => const HomeScreen()),
  GoRoute(path: "/profile", builder: (context, state) => const ProfileScreen())
]);