module day03
    implicit none
    private
  
    public :: day03_parts_1_and_2
  contains
    subroutine day03_parts_1_and_2
      integer :: io
      character(32) :: line
      integer :: idx1, idx2, id, left, top, width, height

      integer :: i, j
      integer, allocatable :: claims_count(:, :)

      logical :: overlaps

      allocate(claims_count(1024, 1024))
      claims_count = 0

      ! Read input
      open(newunit=io, file="inputs/input03", status="old", action="read")
      do
        read(io, "(a)", end=100) line

        idx1 = index(line, "@")
        idx2 = index(line, ",")
        read(line(idx1+1:idx2-1), "(i4)") left

        idx1 = index(line, ":")
        read(line(idx2+1:idx1-1), "(i4)") top

        idx2 = index(line, "x")
        read(line(idx1+1:idx2-1), "(i3)") width

        read(line(idx2+1:), "(i3)") height

        ! Part 1
        do i = left + 1, left + 1 + width - 1
          do j = top + 1, top + 1 + height - 1
            claims_count(i, j) = claims_count(i, j) + 1
          end do
        end do
      end do
      100 close(io)
      print *, "Part 1: ", count(claims_count > 1)

      ! Part 2
      open(newunit=io, file="inputs/input03", status="old", action="read")
      do
        read(io, "(a)", end=200) line

        idx1 = index(line, "@")
        read(line(2:idx1-1), "(i4)") id

        idx2 = index(line, ",")
        read(line(idx1+1:idx2-1), "(i4)") left

        idx1 = index(line, ":")
        read(line(idx2+1:idx1-1), "(i4)") top

        idx2 = index(line, "x")
        read(line(idx1+1:idx2-1), "(i3)") width

        read(line(idx2+1:), "(i3)") height

        ! Part 1
        overlaps = .false.
        do i = left + 1, left + 1 + width - 1
          do j = top + 1, top + 1 + height - 1
            if (claims_count(i, j) /= 1) then
              overlaps = .true.
            end if
          end do
        end do
        if (overlaps .eqv. .false.) then
          print *, "Part 2: ", id
          exit
        end if
      end do
      200 close(io)

    end subroutine day03_parts_1_and_2
end module day03
