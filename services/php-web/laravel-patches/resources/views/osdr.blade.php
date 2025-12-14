@extends('layouts.app')

@section('content')
<div class="container py-3">
  <h3 class="mb-3">Данные NASA OSDR</h3>
  <div class="small text-muted mb-2">Источник {{ $src }}</div>

  <div class="card mb-3 p-3">
    <form action="/osdr" method="GET" class="row g-3">
        <div class="col-md-3">
            <select name="filter_col" class="form-select">
                <option value="title">Название</option>
                <option value="dataset_id">ID Набора</option>
                <option value="status">Статус</option>
            </select>
        </div>
        <div class="col-md-6">
            <input type="text" name="filter_val" class="form-control" placeholder="Значение фильтра...">
        </div>
        <div class="col-md-3">
            <button type="submit" class="btn btn-primary w-100">Фильтровать</button>
        </div>
    </form>
  </div>

  <div class="table-responsive">
    <table class="table table-sm table-striped align-middle table-hover">
      <thead class="table-dark">
        <tr>
          <th><a href="?sort=id&dir={{ $dir === 'asc' ? 'desc' : 'asc' }}" class="text-white text-decoration-none"># {{ $sort === 'id' ? ($dir === 'asc' ? '↑' : '↓') : '' }}</a></th>
          <th><a href="?sort=dataset_id&dir={{ $dir === 'asc' ? 'desc' : 'asc' }}" class="text-white text-decoration-none">dataset_id {{ $sort === 'dataset_id' ? ($dir === 'asc' ? '↑' : '↓') : '' }}</a></th>
          <th><a href="?sort=title&dir={{ $dir === 'asc' ? 'desc' : 'asc' }}" class="text-white text-decoration-none">title {{ $sort === 'title' ? ($dir === 'asc' ? '↑' : '↓') : '' }}</a></th>
          <th>REST_URL</th>
          <th><a href="?sort=updated_at&dir={{ $dir === 'asc' ? 'desc' : 'asc' }}" class="text-white text-decoration-none">updated_at {{ $sort === 'updated_at' ? ($dir === 'asc' ? '↑' : '↓') : '' }}</a></th>
          <th><a href="?sort=inserted_at&dir={{ $dir === 'asc' ? 'desc' : 'asc' }}" class="text-white text-decoration-none">inserted_at {{ $sort === 'inserted_at' ? ($dir === 'asc' ? '↑' : '↓') : '' }}</a></th>
          <th>raw</th>
        </tr>
      </thead>
      <tbody>
      @forelse($items as $row)
        <tr>
          <td>{{ $row['id'] }}</td>
          <td>{{ $row['dataset_id'] ?? '—' }}</td>
          <td style="max-width:420px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">
            {{ $row['title'] ?? '—' }}
          </td>
          <td>
            @if(!empty($row['rest_url']))
              <a href="{{ $row['rest_url'] }}" target="_blank" rel="noopener">открыть</a>
            @else — @endif
          </td>
          <td>{{ $row['updated_at'] ?? '—' }}</td>
          <td>{{ $row['inserted_at'] ?? '—' }}</td>
          <td>
            <button class="btn btn-outline-secondary btn-sm" data-bs-toggle="collapse" data-bs-target="#raw-{{ $row['id'] }}-{{ md5($row['dataset_id'] ?? (string)$row['id']) }}">JSON</button>
          </td>
        </tr>
        <tr class="collapse" id="raw-{{ $row['id'] }}-{{ md5($row['dataset_id'] ?? (string)$row['id']) }}">
          <td colspan="7">
            <pre class="mb-0" style="max-height:260px;overflow:auto">{{ json_encode($row['raw'] ?? [], JSON_PRETTY_PRINT|JSON_UNESCAPED_SLASHES|JSON_UNESCAPED_UNICODE) }}</pre>
          </td>
        </tr>
      @empty
        <tr><td colspan="7" class="text-center text-muted">нет данных</td></tr>
      @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection