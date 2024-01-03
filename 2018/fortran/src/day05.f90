module day05
    implicit none
    private
  
    public :: day05_parts_1_and_2
  contains
    subroutine react_polymer(polymer)
      character(50000), intent(out) :: polymer

      logical :: units_destroyed
      integer :: i

      do
        units_destroyed = .false.
        
        do i = 1, len_trim(polymer) - 1
          if ((ichar(polymer(i:i)) == ichar(polymer(i+1:i+1)) + 32) .or. (ichar(polymer(i:i)) + 32 == ichar(polymer(i+1:i+1)))) then
            polymer = polymer(1:i-1)//polymer(i+2:)
            units_destroyed = .true.
          end if
        end do

        if (.not. units_destroyed) then
          exit
        end if
      end do

      return
    end subroutine

    subroutine replace_unit(unit_upper, unit_lower, polymer)
      character, intent(in) :: unit_upper, unit_lower
      character(50000), intent(out) :: polymer

      logical :: units_removed
      integer :: i

      do 
        units_removed = .false.

        do i = 1, len_trim(polymer)
          if ((polymer(i:i) == unit_upper) .or. (polymer(i:i) == unit_lower)) then
            polymer = polymer(1:i-1)//polymer(i+1:)
            units_removed = .true.
          end if
        end do

        if (.not. units_removed) then
          exit
        end if
      end do

      return
    end subroutine

    subroutine day05_parts_1_and_2
      integer :: io
      character(50000) :: polymer, reacted_polymer, improved_polymer
      character(50000) :: test

      integer :: i
      integer :: shortest_polymer_length

      ! Read input
      open(newunit=io, file="inputs/input05", status="old", action="read")
      read(io, "(a)") polymer
      close(io)

      ! Part 1
      reacted_polymer = polymer
      call react_polymer(reacted_polymer)
      print *, "Part 1: ", len(trim(reacted_polymer))

      ! Part 2
      shortest_polymer_length = 50000
      do i = ichar("A"), ichar("Z")
        improved_polymer = polymer
        call replace_unit(char(i), char(i + 32), improved_polymer)
        call react_polymer(improved_polymer)
        if (len_trim(improved_polymer) < shortest_polymer_length) then
          shortest_polymer_length = len_trim(improved_polymer)
        end if
      end do
      print *, "Part 2: ", shortest_polymer_length

    end subroutine day05_parts_1_and_2
end module day05
