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
    st.markdown("Fire Engineering Calculations using OpenFire")
    
    # Sidebar for navigation
    st.sidebar.title("Navigation")
    page = st.sidebar.selectbox(
        "Select a calculation:",
        ["Welcome", "Heat Release Rate", "Smoke Layer Analysis", "About"]
    )
    
    if page == "Welcome":
        welcome_page()
    elif page == "Heat Release Rate":
        heat_release_rate_page()
    elif page == "Smoke Layer Analysis":
        smoke_filling_page()
    elif page == "About":
        about_page()


def welcome_page():
    """Display the welcome page."""
    st.header("Welcome to {project_name}")
    
    st.markdown("""
    This fire engineering tool provides calculations for:
    
    - **Heat Release Rate**: Calculate HRR at flashover using CIBSE Guide E
    - **Smoke Layer Analysis**: Calculate smoke layer interface height and properties using Fire Dynamics Tools
    - **Custom Calculations**: Add your own fire engineering calculations
    
    Select a calculation from the sidebar to get started.
    """)
    
    st.info("💡 This tool is built using the OpenFire library for fire engineering calculations.")
    
    # Display available modules
    with st.expander("Available OpenFire Modules"):
        st.markdown("""
        - `ofire.br_187`: BR 187 calculations
        - `ofire.bs9999`: BS 9999 calculations  
        - `ofire.cibse_guide_e`: CIBSE Guide E calculations
        - `ofire.fire_dynamics_tools`: General fire dynamics tools
        - `ofire.pd_7974`: PD 7974 calculations
        - `ofire.sfpe_handbook`: SFPE Handbook calculations
        - `ofire.tr_17`: TR 17 calculations
        - `ofire.introduction_to_fire_dynamics`: Introductory calculations
        """)


def heat_release_rate_page():
    """Heat release rate calculation page."""
    st.header("Heat Release Rate at Flashover")
    st.markdown("Calculate the heat release rate at flashover using CIBSE Guide E, Chapter 6.")
    
    col1, col2 = st.columns([1, 1])
    
    with col1:
        st.subheader("Input Parameters")
        room_area = st.number_input(
            "Room Floor Area (m²)",
            min_value=1.0,
            max_value=10000.0,
            value=50.0,
            step=1.0,
            help="Floor area of the room in square meters"
        )
        
        room_height = st.number_input(
            "Room Height (m)",
            min_value=1.0,
            max_value=50.0,
            value=3.0,
            step=0.1,
            help="Height of the room in meters"
        )
        
        if st.button("Calculate HRR", type="primary"):
            try:
                # Example calculation - replace with actual ofire function when available
                # hrr = ofire.cibse_guide_e.chapter_6.equation_6_7.heat_release_rate_flashover(
                #     room_area, room_height
                # )
                
                # Placeholder calculation for demonstration
                hrr = room_area * room_height * 250  # Simplified example
                
                st.session_state.hrr_result = hrr
                st.success("Calculation completed!")
                
            except Exception as e:
                st.error(f"Calculation error: {{e}}")
    
    with col2:
        st.subheader("Results")
        if hasattr(st.session_state, 'hrr_result'):
            st.metric(
                "Heat Release Rate at Flashover",
                f"{{st.session_state.hrr_result:,.0f}} kW",
                help="Heat release rate at the onset of flashover"
            )
            
            # Additional information
            st.info("""
            **Note**: This calculation is based on CIBSE Guide E methodology.
            Replace the placeholder calculation with the actual OpenFire function
            when implementing your specific requirements.
            """)
        else:
            st.info("Enter room parameters and click 'Calculate HRR' to see results.")


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
                # Calculate room floor area
                floor_area = room_length * room_width
                
                # Calculate hot gas density using OpenFire
                hot_gas_density = ofire.fire_dynamics_tools.chapter_2.equation_2_13.density_hot_gas_layer(
                    hot_gas_temp
                )
                
                # Calculate entrainment coefficient using simplified method
                k_coefficient = ofire.fire_dynamics_tools.chapter_2.equation_2_12.k_constant_smoke_layer_height_post_substitution(
                    hot_gas_density
                )
                
                # Calculate smoke layer interface height using Yamana-Tanaka correlation
                interface_height = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation(
                    k_coefficient, heat_release_rate, time_after_ignition, floor_area, room_height
                )
                
                # Calculate derived properties
                smoke_layer_depth = room_height - interface_height
                smoke_volume = floor_area * max(0, smoke_layer_depth)
                percent_filled = (max(0, smoke_layer_depth) / room_height) * 100
                
                # Store results in session state
                st.session_state.smoke_results = {{
                    'hot_gas_density': hot_gas_density,
                    'k_coefficient': k_coefficient,
                    'interface_height': interface_height,
                    'smoke_layer_depth': smoke_layer_depth,
                    'smoke_volume': smoke_volume,
                    'percent_filled': percent_filled,
                    'floor_area': floor_area
                }}
                
                st.success("Smoke layer analysis completed!")
                
            except Exception as e:
                st.error(f"Calculation error: {{e}}")
                st.info("Note: Ensure all parameters are within reasonable ranges for fire engineering calculations.")
    
    with col2:
        st.subheader("Results")
        if hasattr(st.session_state, 'smoke_results'):
            results = st.session_state.smoke_results
            
            # Primary results
            st.metric(
                "Smoke Layer Interface Height",
                f"{{results['interface_height']:.2f}} m",
                help="Height of the interface between clear air below and smoke layer above"
            )
            
            st.metric(
                "Smoke Layer Depth",
                f"{{results['smoke_layer_depth']:.2f}} m",
                help="Thickness of the smoke layer from interface to ceiling"
            )
            
            st.metric(
                "Room Smoke-Filled",
                f"{{results['percent_filled']:.1f}}%",
                help="Percentage of room height filled with smoke"
            )
            
            # Additional properties in expandable section
            with st.expander("Advanced Properties"):
                col3, col4 = st.columns([1, 1])
                
                with col3:
                    st.metric("Hot Gas Density", f"{{results['hot_gas_density']:.3f}} kg/m³")
                    st.metric("Floor Area", f"{{results['floor_area']:.1f}} m²")
                
                with col4:
                    st.metric("Entrainment Coefficient", f"{{results['k_coefficient']:.6f}}")
                    st.metric("Smoke Volume", f"{{results['smoke_volume']:.1f}} m³")
            
            
            # Methodology information
            st.info("""
            **Calculation Method**: 
            - Yamana-Tanaka correlation for smoke layer interface height
            - Natural ventilation conditions assumed
            - Based on Fire Dynamics Tools, Chapter 2
            """)
            
        else:
            st.info("Enter room and fire parameters, then click 'Calculate Smoke Layer Properties' to see results.")
            
            st.markdown("""
            **This calculation determines:**
            - Smoke layer interface height
            - Smoke layer depth and volume
            - Hot gas layer properties
            - Safety assessment for evacuation
            
            **Based on OpenFire implementation of:**
            - Fire Dynamics Tools Chapter 2 equations
            - Yamana-Tanaka correlation for transient conditions
            """)


def about_page():
    """Display information about the application."""
    st.header("About {project_name}")
    
    st.markdown("""
    This fire engineering tool was created using:
    
    - **OpenFire Library**: Comprehensive fire engineering calculations
    - **Python**: Programming language for scientific computing
    - **Web Interface**: Modern interactive calculation interface
    
    ### How to Extend This Tool
    
    1. Add new calculation pages by creating functions like `your_calculation_page()`
    2. Add the new page to the sidebar navigation
    3. Implement calculations using OpenFire library functions
    4. Use web interface components for interactive inputs and results display
    
    ### OpenFire Documentation
    
    Visit [OpenFire Documentation](https://emberon-tech.github.io/openfire/) for detailed
    information about available calculations and usage examples.
    """)
    
    st.markdown("---")
    st.markdown("Built with ❤️ using OpenFire")


if __name__ == "__main__":
    main()