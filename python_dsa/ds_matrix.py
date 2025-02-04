def col_sorted_matri(matrix, k):
    for row in matrix:
        for item in row:
            if(item == k):
                return True
        
    return False
def spiral_matrix(matrix):
    if not matrix or not matrix[0]:  
        return []

    result = []
    top, bottom = 0, len(matrix) - 1
    left, right = 0, len(matrix[0]) - 1

    while top <= bottom and left <= right:
        # move from left to right
        for i in range(left, right + 1):
            result.append(matrix[top][i])
        top += 1  # Move the top boundary down

        # move from top to bottom
        for i in range(top, bottom + 1):
            result.append(matrix[i][right])
        right -= 1  # Move the right boundary left

        # move from right to left (only if there's a new row)
        if top <= bottom:
            for i in range(right, left - 1, -1):
                result.append(matrix[bottom][i])
            bottom -= 1  # Move the bottom boundary up

        # move from bottom to top (only if there's a new column)
        if left <= right:
            for i in range(bottom, top - 1, -1):
                result.append(matrix[i][left])
            left += 1  # move the left boundary right

    return result
            
print(spiral_matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]))