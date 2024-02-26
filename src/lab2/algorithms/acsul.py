"""

    Алгоритм построения расписания с нестабильной загрузкой


"""
import random


def main(matrix: list[list[int]]):
    while True:
        try:
            sort_mode = int(
                input(
                    "Как отсортировать?"
                    "\n - 1: По убыванию;"
                    "\n - 2. По возрастанию;"
                    "\n - 3. Случайный порядок "
                    "\n\nВведите номер: "
                )
            )
            if sort_mode not in [1, 2, 3]:
                raise ValueError
            break
        except ValueError:
            print("Ошибка ввода, попробуйте снова")

    if sort_mode in [1, 2]:
        matrix.sort(key=lambda x: sum(x), reverse=sort_mode == 1)
    else:
        random.shuffle(matrix)

    print("\nОтсортированная матрица:")
    for row in matrix:
        print("\t".join(map(str, row)), f"\t | Сумма: {sum(row)}")

    last_values = [0 for _ in range(len(matrix[0]))]

    print("\n Решение:")
    for row_index, row in enumerate(matrix):
        minimum = min(row)
        col_index = row.index(minimum)

        last_values[col_index] = minimum

        print(f"\nШаг {row_index + 1}:")
        for i in range(len(matrix)):
            if i == row_index:
                els = list(map(str, matrix[i]))
                els[col_index] = f"\033[1;31m{els[col_index]}\033[0m"
            elif i > row_index:
                els = list(map(str, matrix[i]))
                els[col_index] = f"\033[1;32m{els[col_index]}\033[0m"
                matrix[i][col_index] += minimum
            else:
                els = list(map(str, matrix[i]))

            print("\t".join(els))
    print("\nРезультат:")
    print("\t".join(map(str, last_values)), f"\t | Max: {max(last_values)}")


if __name__ == "__main__":
    main(
        [
            [17, 14, 12],
            [9, 5, 11],
            [15, 7, 12],
            [13, 10, 8],
            [11, 16, 14]
        ]
    )
