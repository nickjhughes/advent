module day01
  use stdlib_kinds, only: int8, int32
  use stdlib_hashmaps, only: chaining_hashmap_type
  use stdlib_hashmap_wrappers, only: fnv_1_hasher, key_type, set
  
  implicit none
  private

  public :: day01_parts_1_and_2
contains
  function int32_to_int8_vector(a) result(b)
    integer(int32) :: a
    integer(int8) :: b(4)
    b = [iand(a, 255), iand(ishft(a, -8), 255), iand(ishft(a, -16), 255), iand(ishft(a, -24), 255)]
  end function

  subroutine day01_parts_1_and_2
    integer :: io
    integer, allocatable :: changes(:)
    integer :: change

    integer(int32) :: frequency
    integer :: i

    type(chaining_hashmap_type) :: frequencies_map
    type(key_type) :: frequency_key
    logical :: frequency_seen

    ! Read input
    open(newunit=io, file="inputs/input01", status="old", action="read")
    allocate(changes(0))
    do while (.true.)
      read(io, *, end=100) change
      changes = [changes, change]
    end do
    100 close(io)

    ! Part 1
    frequency = 0
    do i = 1, size(changes)
      frequency = frequency + changes(i)
    end do
    print *, "Part 1: ", frequency

    ! Part 2
    call frequencies_map%init(fnv_1_hasher)
    frequency = 0
    i = 1
    do while (.true.)
      frequency = frequency + changes(i)

      call set(frequency_key, int32_to_int8_vector(frequency))
      call frequencies_map%key_test(frequency_key, frequency_seen)
      if (frequency_seen) then
        print *, "Part 2: ", frequency
        exit
      end if
      call frequencies_map%map_entry(frequency_key)

      i = i + 1
      if (i > size(changes)) then
        i = 1
      end if
    end do
  end subroutine day01_parts_1_and_2
end module day01
