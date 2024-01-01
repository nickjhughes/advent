module day01
  implicit none
  private

  public :: day01_parts_1_and_2
contains
  subroutine day01_parts_1_and_2
    integer :: io
    integer, allocatable :: changes(:)
    integer :: change

    integer :: frequency
    integer :: i

    integer, allocatable :: frequencies(:)

    ! Read input
    open(newunit=io, file="inputs/day1", status="old", action="read")
    allocate(changes(0))
    do while (.true.)
      read(io, *, end=100) change
      changes = [changes, change]
    end do
    100 close(io)

    ! Part 1, add all changes
    frequency = 0
    do i = 1, size(changes)
      frequency = frequency + changes(i)
    end do
    print *, "Part 1: ", frequency

    ! Part 2
    frequency = 0
    i = 1
    do while (.true.)
      frequency = frequency + changes(i)
      if (any(frequencies == frequency)) then
        print *, "Part 2: ", frequency
        exit
      end if
      frequencies = [frequencies, frequency]
      i = i + 1
      if (i > size(changes)) then
        i = 1
      end if
    end do
  end subroutine day01_parts_1_and_2
end module day01
