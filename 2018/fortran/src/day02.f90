module day02
    implicit none
    private
  
    public :: day02_parts_1_and_2
  contains
    subroutine day02_parts_1_and_2
      integer :: io
      character(26), allocatable :: box_ids(:)
      character(26) :: box_id
      
      integer :: i, j, k
      integer :: letter_count(26)
      integer :: letter_index
      integer :: two_count, three_count
      integer :: difference_index
  
      ! Read input
      open(newunit=io, file="inputs/input02", status="old", action="read")
      allocate(box_ids(1))
      read(io, "(26a)", end=100) box_id
      box_ids(1) = box_id
      do while (.true.)
        read(io, "(26a)", end=100) box_id
        box_ids = [box_ids, box_id]
      end do
      100 close(io)
  
      ! Part 1
      two_count = 0
      three_count = 0
      do i = 1, size(box_ids)
        letter_count = 0
        do j = 1, 26
          letter_index = ichar(box_ids(i)(j:j)) - 97 + 1
          letter_count(letter_index) = letter_count(letter_index) + 1
        end do

        if (any(letter_count == 2)) then
          two_count = two_count + 1
        end if
        if (any(letter_count == 3)) then
          three_count = three_count + 1
        end if
      end do
      print *, "Part 1: ", two_count * three_count

      ! Part 2
      do i = 1, size(box_ids)
        do j = i + 1, size(box_ids)
          difference_index = 0
          do k = 1, 26
            if (box_ids(i)(k:k) /= box_ids(j)(k:k)) then
              if (difference_index /= 0) then
                difference_index = 0
                exit
              else
                difference_index = k
              end if
            end if
          end do
          if (difference_index /= 0) then
            write(*, fmt="(a)", advance="no") " Part 2: "
            do k = 1, 26
              if (k /= difference_index) then
                write(*, fmt="(a)", advance="no") box_ids(i)(k:k)
              end if
            end do
            write(*, *)
          end if
        end do
      end do
    end subroutine day02_parts_1_and_2
  end module day02
  