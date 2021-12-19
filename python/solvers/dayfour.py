def score(expl_board, draw_set, last_draw):
    return int(last_draw) * sum(
        int(s) for row in expl_board["rows"] for s in row if s not in draw_set
    )


def board_wins(expl_board, draw_set):
    return any(
        set(line) <= draw_set for line in expl_board["rows"] + expl_board["cols"]
    )


def explode_board(board):
    return {
        "rows": board,
        "cols": [[row[i] for row in board] for i in range(len(board[0]))],
    }


def collect_boards(lines):
    boards = []
    board = []
    for line in lines:
        if line.strip() == "":
            boards.append(board)
            board = []
        else:
            board.append(line.split())

    return [explode_board(b) for b in boards]


def part_one(lines):
    draws = lines[0].split(",")
    boards = collect_boards(lines[2:])

    draw_set = set()
    for draw in draws:
        draw_set.add(draw)
        winner = next((b for b in boards if board_wins(b, draw_set)), None)
        if winner:
            return score(winner, draw_set, draw)

    raise Exception("Nobody wins!")


def part_two(lines):
    draws = lines[0].split(",")
    boards = collect_boards(lines[2:])

    draw_set = set()
    for draw in draws:
        draw_set.add(draw)
        non_winners = [b for b in boards if not board_wins(b, draw_set)]
        if len(non_winners) == 0:
            return score(boards[0], draw_set, draw)
        else:
            boards = non_winners

    raise Exception("Some board(s) never won!")
