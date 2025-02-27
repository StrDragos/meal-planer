import 'package:mobile/common/model/summary_dish.dart';
import 'package:mobile/common/repositories/dishes_repo.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'get_ternding_receipts_use_case.g.dart';

@riverpod
Future<List<SummaryDish>> getTrendingReceipts(Ref ref) async {
  final repo = ref.watch(dishRepoProvider);
  return repo.trendingReceipts();
}
