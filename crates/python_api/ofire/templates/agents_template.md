# AI Agent Guide

## Technology Stack
- **Language**: Python 3.8+
- **UI Framework**: Streamlit
- **Fire Engineering Library**: OpenFire (ofire)
- **Environment**: Virtual environment (.venv)

## Key Rules
1. **Always use ofire library** for fire engineering calculations
2. **Use Streamlit** for user interfaces
3. **Follow patterns** in main.py

## Run Application
```bash
ofire run
```

## UI Pattern
```python
import streamlit as st
import ofire

# Two column layout: inputs left, results right
col1, col2 = st.columns([1, 1])

with col1:
    param = st.number_input("Parameter", value=10.0)
    if st.button("Calculate"):
        result = ofire.module.function(param)
        st.session_state.result = result

with col2:
    if hasattr(st.session_state, 'result'):
        st.metric("Result", f"{st.session_state.result:.2f}")
```