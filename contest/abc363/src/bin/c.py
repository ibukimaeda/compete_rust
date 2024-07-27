from itertools import permutations

def is_valid_permutation(perm, K):
    # 文字列の任意の部分文字列が回文であるかをチェック
    for i in range(len(perm) - K + 1):
        substring = perm[i:i + K]
        if substring == substring[::-1]:
            return False
    return True

def count_valid_permutations(N, K, S):
    perm_set = set(permutations(S))  # すべての並べ替えを生成し、重複を取り除く
    count = 0
    
    for perm in perm_set:
        perm_str = ''.join(perm)
        if is_valid_permutation(perm_str, K):
            count += 1
            
    return count

# 入力
N = int(input())
K = int(input())
S = input().strip()

# 結果の計算と出力
result = count_valid_permutations(N, K, S)
print(result)
