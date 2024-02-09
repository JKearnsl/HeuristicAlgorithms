"""

    Алгоритм критического пути

"""


def main(matrix: list[list[int]]) -> None:
    column = [0] * len(matrix)
    for i in range(len(matrix)):
        column[i] = matrix[i][0]

    times = sorted(column, reverse=True)
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            matrix[i][j] = times[i]

    print("\nОтсортированная матрица:")
    for row in matrix:
        print("\t".join(map(str, row)))

    processors = [[] for _ in range(len(matrix[0]))]

    step = 0
    for time in times:
        step += 1
        min_load_processor = min(range(len(processors)), key=lambda index: sum(processors[index]))
        processors[min_load_processor].append(time)

        print(f"Шаг {step}:")
        for i, processor in enumerate(processors):
            print(f"Процессор {i + 1}: {processor}", "\t", sum(processor))
        print(f"Максимальная нагрузка: {max(sum(processor) for processor in processors)}, выполнено за {step} шагов")
