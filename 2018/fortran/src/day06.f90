module day06
    implicit none
    private
  
    public :: day06_parts_1_and_2
  contains
    subroutine day06_parts_1_and_2
      type point
        integer :: x
        integer :: y
      end type point

      integer :: io, idx
      character(8) :: line
      type(point) :: cur_point
      type(point), allocatable :: points(:)

      integer :: i, min_x, min_y, max_x, max_y
      integer, allocatable :: grid(:, :)
      integer :: y, x, dist, closest_dist, closest_point, closest_point_count
      integer :: largest_region_size

      integer :: total_dist

      ! Read input
      open(newunit=io, file="inputs/input06", status="old", action="read")
      allocate(points(0))
      do
        read(io, "(a)", end=100) line

        idx = index(line, ", ")
        read(line(1:idx-1), "(i3)") cur_point%x
        read(line(idx+2:), "(i3)") cur_point%y

        points = [points, cur_point]
      end do
      100 close(io)

      ! Part 1
      min_x = 1000
      min_y = 1000
      max_x = 0
      max_y = 0
      do i = 1, size(points)
        if (points(i)%x < min_x) then
          min_x = points(i)%x
        end if
        if (points(i)%y < min_y) then
          min_y = points(i)%y
        end if
        if (points(i)%x > max_x) then
          max_x = points(i)%x
        end if
        if (points(i)%y > max_y) then
          max_y = points(i)%y
        end if
      end do
      min_x = min_x - 1
      min_y = min_y - 1
      max_x = max_x + 1
      max_y = max_y + 1

      allocate(grid(max_y - min_y + 1, max_x - min_x + 1))
      do y = min_y, max_y
        do x = min_x, max_x
          closest_dist = 10000
          closest_point = 0
          do i = 1, size(points)
            dist = abs(points(i)%x - x) + abs(points(i)%y - y)
            if (dist < closest_dist) then
              closest_dist = dist
              closest_point = i
            end if
          end do

          closest_point_count = 0
          do i = 1, size(points)
            dist = abs(points(i)%x - x) + abs(points(i)%y - y)
            if (dist == closest_dist) then
              closest_point_count = closest_point_count + 1
              if (closest_point_count > 1) then
                exit
              end if
            end if
          end do
          if (closest_point_count > 1) then
            closest_point = 0
          end if

          grid(y - min_y + 1, x - min_x + 1) = closest_point
        end do
      end do

      largest_region_size = 0
      do i = 1, size(points)
        if (any(grid(1,:) == i) .or. any(grid(size(grid,1),:) == i)) then
          cycle
        end if
        if (any(grid(:,1) == i) .or. any(grid(:,size(grid,2)) == i)) then
          cycle
        end if

        if (count(grid == i) > largest_region_size) then
          largest_region_size = count(grid == i)
        end if
      end do
      print *, "Part 1: ", largest_region_size

      ! Part 2
      grid = 0
      do y = min_y, max_y
        do x = min_x, max_x
          total_dist = 0
          do i = 1, size(points)
            total_dist = total_dist + abs(points(i)%x - x) + abs(points(i)%y - y)
          end do
          if (total_dist < 10000) then
            grid(y - min_y + 1, x - min_x + 1) = 1
          end if
        end do
      end do
      print *, "Part 2: ", count(grid == 1)

    end subroutine day06_parts_1_and_2
end module day06
