"""Check externally visible observations and non-overlapping phase arithmetic."""
import json

QUERY_PHASES = ['query_setup_ns', 'observer_setup_ns', 'service_ns', 'pending_drop_ns',
                'export_ns', 'query_drop_ns']
SESSION_PHASES = ['runtime_init_ns', 'source_load_ns', 'prepare_ns', 'source_drop_ns',
                  'consumer_setup_ns', 'prepared_drop_ns', 'consumer_drop_ns']

def check(plan, row, expected):
    assert row['code'] == 0, row['stderr'][-2000:]
    assert row['stdout'] == expected['stdout'], plan['group']
    events = [json.loads(line) for line in row['stderr'].splitlines()]
    prior = [json.loads(line) for line in expected['stderr'].splitlines()]
    assert len(events) == len(prior)
    for event, old in zip(events[:-1], prior[:-1]):
        for key in ['query', 'calls', 'pending', 'unsupported', 'dynamic_words', 'retained_bytes']:
            assert event[key] == old[key], (plan['group'], key)
        assert event['allocator'] == 'ordinary'
        for phase in QUERY_PHASES + ['serialization_ns', 'compute_traverse_ns', 'consumer_drop_ns']:
            assert type(event[phase]) is int and event[phase] >= 0, phase
        assert event['service_ns'] == event['serialization_ns'] + event['compute_traverse_ns']
        assert event['query_total_ns'] == sum(event[p] for p in QUERY_PHASES)
        first = event['first_observation_ns']
        assert (first is not None) == (event['retained_bytes'] > 1)
        if first is not None:
            assert 0 <= first <= event['service_ns']
    end = events[-1]
    assert end['session_disposed'] and end['allocator'] == 'ordinary'
    assert end['prepared_words'] == prior[-1]['prepared_words']
    assert end['peak_dynamic_words'] == prior[-1]['peak_dynamic_words']
    for phase in SESSION_PHASES:
        assert type(end[phase]) is int and end[phase] >= 0, phase
    assert end['lifecycle_ns'] == sum(end[p] for p in SESSION_PHASES) + sum(e['query_total_ns'] + e['consumer_drop_ns'] for e in events[:-1])
    return events
