@extends('layouts.app')

@section('content')
<div class="container py-3">
  <h3 class="mb-3">Архив Телеметрии (Legacy)</h3>
  <div class="card p-3 mb-3">
      <p>Данные сгенерированы Pascal Legacy Service. Визуализация по запросу.</p>
  </div>

  <div class="table-responsive">
    <table class="table table-sm table-striped align-middle table-hover">
      <thead class="table-dark">
        <tr>
          <th>ID</th>
          <th>Записано (UTC)</th>
          <th>Напряжение (V)</th>
          <th>Темп. (°C)</th>
          <th>Активен</th>
          <th>Статус</th>
          <th>Файл источника</th>
        </tr>
      </thead>
      <tbody>
      @forelse($items as $item)
        <tr>
          <td>{{ $item->id }}</td>
          <td>{{ $item->recorded_at }}</td>
          <td>{{ $item->voltage }}</td>
          <td>{{ $item->temp }}</td>
          <td>
              @if(isset($item->is_active))
                <span class="badge bg-{{ $item->is_active ? 'success' : 'danger' }}">
                    {{ $item->is_active ? 'TRUE' : 'FALSE' }}
                </span>
              @else
                <span class="text-muted">—</span>
              @endif
          </td>
          <td>{{ $item->status_message ?? '—' }}</td>
          <td><code>{{ $item->source_file }}</code></td>
        </tr>
      @empty
        <tr><td colspan="7" class="text-center text-muted">No legacy data found</td></tr>
      @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection