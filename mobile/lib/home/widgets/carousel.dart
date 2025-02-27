import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile/common/model/summary_dish.dart';
import 'package:mobile/common/use_cases/get_ternding_receipts_use_case.dart';
import 'package:mobile/home/widgets/carousel_dish_card.dart';

class DishCarousel extends ConsumerWidget {
  const DishCarousel({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final AsyncValue<List<SummaryDish>> receiptTrends = ref.watch(getTrendingReceiptsProvider);
    return receiptTrends.when(
      data: (receipts) {
        return Column(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  "Trending receipts",
                  style: Theme.of(context).textTheme.titleMedium,
                ),
                TextButton(onPressed: () {}, child: const Text("See All"))
              ],
            ),
            SizedBox(
              height: 192,
              child: CarouselView(
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
                padding: const EdgeInsets.only(right: 16, bottom: 16),
                elevation: 0.5,
                itemSnapping: true,
                itemExtent: MediaQuery.sizeOf(context).width / 2,
                shrinkExtent: MediaQuery.sizeOf(context).width / 2,
                children: receipts.map((e) => SummaryDishCard(dish: e)).toList(),
              ),
            )
          ],
        );
      },
      error: (error, _) {
        //TODO: handle error
        return SizedBox.shrink();
      },
      loading: () => const Center(
        child: CircularProgressIndicator(),
      ),
    );
  }
}
