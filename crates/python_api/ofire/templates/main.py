"""
{project_name} - Fire Engineering Tool

A web application for fire engineering calculations
using the OpenFire library.
"""

import streamlit as st
import ofire


def main():
    """Main Streamlit application."""
    st.set_page_config(
        page_title="{project_name}",
        page_icon="🔥",
        layout="wide",
        initial_sidebar_state="expanded"
    )
    
    st.title("🔥 {project_name}")
    st.markdown("Fire Engineering Calculations built using OpenFire")
    
    # Sidebar for navigation
    st.sidebar.title("Navigation")
    
    # Initialize session state for page selection
    if 'current_page' not in st.session_state:
        st.session_state.current_page = "Welcome"
    
    # Create navigation buttons
    if st.sidebar.button("🏠 Welcome", use_container_width=True):
        st.session_state.current_page = "Welcome"
    
    if st.sidebar.button("🔥 Smoke Layer Analysis", use_container_width=True):
        st.session_state.current_page = "Smoke Layer Analysis"
    
    page = st.session_state.current_page
    
    if page == "Welcome":
        welcome_page()
    elif page == "Smoke Layer Analysis":
        smoke_filling_page()


def welcome_page():
    st.header("Welcome to {project_name}")
    
    st.markdown("""
    This fire engineering tool provides calculations for:
    
    - **Smoke Layer Analysis**: Calculate smoke layer interface height and properties using Fire Dynamics Tools
    - **Custom Calculations**: Add your own fire engineering calculations
    
    Select a calculation from the sidebar to get started.
    """)
    
    st.info("💡 This tool is built using the OpenFire library for fire engineering calculations.")
    

def smoke_filling_page():
    """Smoke layer analysis calculation page."""
    st.header("Smoke Layer Analysis")
    st.markdown("Calculate smoke layer interface height and properties using OpenFire fire dynamics tools.")
    
    col1, col2 = st.columns([1, 1])
    
    with col1:
        st.subheader("Room Parameters")
        room_length = st.number_input(
            "Room Length (m)",
            min_value=1.0,
            max_value=100.0,
            value=10.0,
            step=0.1,
            help="Length of the room in meters"
        )
        
        room_width = st.number_input(
            "Room Width (m)",
            min_value=1.0,
            max_value=100.0,
            value=8.0,
            step=0.1,
            help="Width of the room in meters"
        )
        
        room_height = st.number_input(
            "Room Height (m)",
            min_value=1.0,
            max_value=50.0,
            value=3.0,
            step=0.1,
            help="Height of the room in meters"
        )
        
        st.subheader("Fire Parameters")
        heat_release_rate = st.number_input(
            "Heat Release Rate (kW)",
            min_value=10.0,
            max_value=50000.0,
            value=1000.0,
            step=10.0,
            help="Heat release rate of the fire in kilowatts"
        )
        
        time_after_ignition = st.number_input(
            "Time After Ignition (s)",
            min_value=1.0,
            max_value=3600.0,
            value=90.0,
            step=1.0,
            help="Time elapsed since ignition in seconds"
        )
        
        hot_gas_temp = st.number_input(
            "Hot Gas Temperature (K)",
            min_value=300.0,
            max_value=1500.0,
            value=500.0,
            step=10.0,
            help="Temperature of the hot gas layer in Kelvin"
        )
        
        if st.button("Calculate Smoke Layer Properties", type="primary"):
            try:
                floor_area = room_length * room_width
                
                hot_gas_density = ofire.fire_dynamics_tools.chapter_2.equation_2_13.density_hot_gas_layer(
                    hot_gas_temp
                )
                
                k_coefficient = ofire.fire_dynamics_tools.chapter_2.equation_2_12.k_constant_smoke_layer_height_post_substitution(
                    hot_gas_density
                )
                
                interface_height = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation(
                    k_coefficient, heat_release_rate, time_after_ignition, floor_area, room_height
                )
                
                smoke_layer_depth = room_height - interface_height
                smoke_volume = floor_area * max(0, smoke_layer_depth)
                percent_filled = (max(0, smoke_layer_depth) / room_height) * 100
                
                st.session_state.smoke_results = {
                    'hot_gas_density': hot_gas_density,
                    'k_coefficient': k_coefficient,
                    'interface_height': interface_height,
                    'smoke_layer_depth': smoke_layer_depth,
                    'smoke_volume': smoke_volume,
                    'percent_filled': percent_filled,
                    'floor_area': floor_area
                }
                
                st.success("Smoke layer analysis completed!")
                
            except Exception as e:
                st.error(f"Calculation error: {e}")
                st.info("Note: Ensure all parameters are within reasonable ranges for fire engineering calculations.")
    
    with col2:
        st.subheader("Results")
        if hasattr(st.session_state, 'smoke_results'):
            results = st.session_state.smoke_results
            
            # Primary results
            st.metric(
                "Smoke Layer Interface Height",
                f"{results['interface_height']:.2f} m",
                help="Height of the interface between clear air below and smoke layer above"
            )
            
            st.metric(
                "Smoke Layer Depth",
                f"{results['smoke_layer_depth']:.2f} m",
                help="Thickness of the smoke layer from interface to ceiling"
            )
            
            st.metric(
                "Room Smoke-Filled",
                f"{results['percent_filled']:.1f}%",
                help="Percentage of room height filled with smoke"
            )
            
            
            # Methodology information
            st.info("""
            **Calculation Method**: 
            - Yamana-Tanaka correlation for smoke layer interface height
            - Natural ventilation conditions assumed
            - Based on Fire Dynamics Tools, Chapter 2
            """)
            
        else:
            st.info("Enter room and fire parameters, then click 'Calculate Smoke Layer Properties' to see results.")


if __name__ == "__main__":
    main()