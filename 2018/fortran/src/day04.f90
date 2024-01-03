module day04
    use stdlib_sorting, only: sort

    implicit none
    private
  
    public :: day04_parts_1_and_2
  contains
    subroutine day04_parts_1_and_2
      type :: event
        integer :: month
        integer :: day
        integer :: hour
        integer :: minute
        ! 0 if not a begins shift event
        integer :: guard_id
        ! 1 = begins shift, 2 = falls asleep, 3 = wakes up
        integer :: event_type
      end type

      integer :: io
      character(64), allocatable :: lines(:)
      character(64) :: line
      integer :: i, j, idx
      type(event) :: current_event

      type(event), allocatable :: events(:)
      integer, allocatable :: guard_ids(:)

      logical, allocatable :: sleep(:, :, :)
      integer :: date_index, guard_index
      integer :: current_guard_id
      integer :: sleep_start_minute

      integer :: sleep_minutes, sleep_days
      integer :: sleepiest_guard_id, sleepiest_guard_minutes
      integer :: sleepiest_minute, sleepiest_guard_days

      ! Read input
      open(newunit=io, file="inputs/input04", status="old", action="read")
      allocate(lines(1))
      read(io, "(a)", end=100) lines(1)
      do
        read(io, "(a)", end=100) line
        lines = [lines, line]
      end do
      100 close(io)

      ! Sort and parse events
      call sort(lines)
      allocate(events(0))
      allocate(guard_ids(0))
      do i = 1, size(lines)
        line = lines(i)

        read(line(7:9), "(i2)") current_event%month
        read(line(10:12), "(i2)") current_event%day
        read(line(13:15), "(i2)") current_event%hour
        read(line(16:18), "(i2)") current_event%minute

        if (line(20:) == "falls asleep") then
          current_event%event_type = 2
          current_event%guard_id = 0
        else if (line(20:) == "wakes up") then
          current_event%event_type = 3
          current_event%guard_id = 0
        else
          current_event%event_type = 1
          idx = index(line(27:), " ")
          read(line(27:27+idx-1), "(i4)") current_event%guard_id
          
          if (.not.(any(guard_ids == current_event%guard_id))) then
            guard_ids = [guard_ids, current_event%guard_id]
          end if
        end if

        events = [events, current_event]
      end do

      ! Convert events to sleep array
      allocate(sleep(372, 60, size(guard_ids)))
      sleep = .false.
      do i = 1, size(events)
        select case(events(i)%event_type)
        case(1)
          current_guard_id = events(i)%guard_id
        case(2)
          sleep_start_minute = events(i)%minute
        case(3)
          date_index = events(i)%month * 31 + events(i)%day
          guard_index = findloc(guard_ids, current_guard_id, dim=1)
          sleep(date_index, sleep_start_minute+1:events(i)%minute, guard_index) = .true.
        end select
      end do
      
      ! Part 1
      sleepiest_guard_minutes = 0
      do i = 1, size(guard_ids)
        sleep_minutes = count(sleep(:, :, i))
        if (sleep_minutes > sleepiest_guard_minutes) then
          sleepiest_guard_minutes = sleep_minutes
          sleepiest_guard_id = guard_ids(i)
        end if
      end do
      sleepiest_guard_days = 0

      do i = 1, 60
        sleep_days = count(sleep(:, i, findloc(guard_ids, sleepiest_guard_id, dim=1)))
        if (sleep_days > sleepiest_guard_days) then
          sleepiest_guard_days = sleep_days
          sleepiest_minute = i - 1
        end if
      end do
      print *, "Part 1: ", sleepiest_minute * sleepiest_guard_id

      ! Part 2
      sleepiest_guard_days = 0
      do i = 1, size(guard_ids)
        do j = 1, 60
          sleep_days = count(sleep(:, j, i))
          if (sleep_days > sleepiest_guard_days) then
            sleepiest_guard_days = sleep_days
            sleepiest_guard_id = guard_ids(i)
            sleepiest_minute = j - 1
          end if
        end do
      end do
      print *, "Part 2: ", sleepiest_minute * sleepiest_guard_id
   
    end subroutine day04_parts_1_and_2
end module day04
