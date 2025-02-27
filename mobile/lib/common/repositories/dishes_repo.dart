import 'package:mobile/common/model/summary_dish.dart';
//TODO:remove files like these once the endpoint works
import 'package:mobile/common/repositories/test_image.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'dishes_repo.g.dart';

abstract class DishesRepo {
  Future<List<SummaryDish>> trendingReceipts();
}

class MockedDishRepo implements DishesRepo {
  @override
  Future<List<SummaryDish>> trendingReceipts() async => [
      SummaryDish(
        name: "Pizza",
        preparationMinutes: 30,
        base64Image: image
      ),
      SummaryDish(
          name: "Pizza",
          preparationMinutes: 45,
          base64Image: image
      ),
      SummaryDish(
          name: "Pizza",
          preparationMinutes: 50,
          base64Image: image
      )
    ];
}


@riverpod
DishesRepo dishRepo(Ref ref) {
  return MockedDishRepo();
}