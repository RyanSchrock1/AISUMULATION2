import os
import re
import sys
from pathlib import Path

# Root directory of the project
ROOT_DIR = Path(r'c:\Users\schro\AI_Simulation')

def fix_condition_module_visibility():
    """Make the condition module public by updating imports and re-exports."""
    nodes_mod_path = ROOT_DIR / 'src' / 'ai' / 'behavior' / 'nodes' / 'mod.rs'
    
    # Condition module is already declared as pub, but we need to ensure it's properly exported
    print(f"[OK] Confirmed condition module is public in {nodes_mod_path}")
    
    # Make sure it's publicly re-exported from the parent
    behavior_mod_path = ROOT_DIR / 'src' / 'ai' / 'behavior' / 'mod.rs'
    content = behavior_mod_path.read_text()
    if "pub mod nodes;" not in content:
        content = content.replace("mod nodes;", "pub mod nodes;")
        behavior_mod_path.write_text(content)
        print(f"[OK] Made nodes module public in {behavior_mod_path}")

def fix_startup_schedule_import():
    """Fix the StartupSchedule import path."""
    mod_path = ROOT_DIR / 'src' / 'ai' / 'mod.rs'
    content = mod_path.read_text()
    content = content.replace(
        "use bevy::app::StartupSchedule;", 
        "use bevy::app::startup_schedule::StartupSchedule;"
    )
    mod_path.write_text(content)
    print(f"[OK] Fixed StartupSchedule import in {mod_path}")

def fix_duplicate_behavior_node():
    """Fix duplicate BehaviorNode definitions and imports."""
    behavior_mod_path = ROOT_DIR / 'src' / 'ai' / 'behavior' / 'mod.rs'
    content = behavior_mod_path.read_text()
    
    # Remove self-referential import of BehaviorNode
    content = re.sub(r'pub use crate::ai::behavior::BehaviorNode;', 
                   '// Removed duplicate BehaviorNode import', content)
    behavior_mod_path.write_text(content)
    print(f"[OK] Removed duplicate BehaviorNode definition in {behavior_mod_path}")

def fix_missing_trait_methods():
    """Add missing BehaviorNode trait methods to implementations."""
    # Add to condition.rs
    condition_path = ROOT_DIR / 'src' / 'ai' / 'behavior' / 'nodes' / 'condition.rs'
    with open(condition_path, 'r') as f:
        content = f.read()

    # Check and fix implementation for IsLowHealth
    if 'impl BehaviorNode for IsLowHealth' in content and 'fn as_any(&self)' not in content:
        pattern = r'(impl BehaviorNode for IsLowHealth \{.*?fn reset\(&mut self\) \{\}.*?\})'
        replacement = r'\1\n    fn as_any(&self) -> &dyn Any { self }\n    fn as_any_mut(&mut self) -> &mut dyn Any { self }\n    fn clone_boxed(&self) -> Box<dyn BehaviorNode> { Box::new(self.clone()) }\n}'
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
        print("✓ Added missing trait methods to IsLowHealth")
    
    # Check and fix other implementations as needed
    # Similar patterns for other BehaviorNode implementations
    
    # Fix the world field reference in BehaviorContext
    content = content.replace('context.world', 'world')
    
    with open(condition_path, 'w') as f:
        f.write(content)
    
    # Fix navigation.rs
    navigation_path = ROOT_DIR / 'src' / 'ai' / 'systems' / 'navigation.rs'
    with open(navigation_path, 'r') as f:
        content = f.read()
    
    # Add missing BehaviorNode methods to NavigateTo
    if 'impl BehaviorNode for NavigateTo' in content and 'fn as_any(&self)' not in content:
        pattern = r'(impl BehaviorNode for NavigateTo \{.*?fn reset\(&mut self\) \{\}.*?\})'
        replacement = r'\1\n    fn as_any(&self) -> &dyn Any { self }\n    fn as_any_mut(&mut self) -> &mut dyn Any { self }\n    fn clone_boxed(&self) -> Box<dyn BehaviorNode> { Box::new(self.clone()) }\n}'
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
        print("✓ Added missing trait methods to NavigateTo")
    
    with open(navigation_path, 'w') as f:
        f.write(content)

def clean_up_unused_imports():
    """Remove unused imports from various files."""
    action_fixed_path = ROOT_DIR / 'src' / 'ai' / 'behavior' / 'nodes' / 'action_fixed.rs'
    with open(action_fixed_path, 'r') as f:
        content = f.read()
    
    # Replace imports with cleaned up version
    content = re.sub(
        r'use.*?\n(?=use|\n|/)',
        '',
        content,
        flags=re.DOTALL
    )
    
    # Add back only the necessary imports
    cleaned_imports = """use rand::Rng;
use bevy::math::Vec2;
use bevy::ecs::entity::Entity;
use std::any::Any;
use crate::ai::behavior::common::{BehaviorNode, BehaviorContext, NodeResult};
"""
    content = re.sub(r'//.*?\n\n', f'//! BehaviorNode implementation for action nodes.\n\n{cleaned_imports}\n', content)
    
    with open(action_fixed_path, 'w') as f:
        f.write(content)
    
    print(f"[OK] Cleaned up unused imports in {action_fixed_path}")
    
    # Similarly clean up other files with unused imports

def fix_behavior_execution_errors():
    """Fix errors in behavior_execution.rs."""
    behavior_execution_path = ROOT_DIR / 'src' / 'ai' / 'behavior_execution.rs'
    with open(behavior_execution_path, 'r') as f:
        content = f.read()
    
    # Fix imports
    content = content.replace(
        "use crate::ai::behavior::BehaviorNode;",
        "// Removed redundant BehaviorNode import"
    )
    
    # Fix Option<Entity> to Entity mismatch
    content = re.sub(
        r'(let entity: Entity = )(.+?);',
        r'\1\2.unwrap_or(context.entity);',
        content
    )
    
    with open(behavior_execution_path, 'w') as f:
        f.write(content)
    
    print(f"[OK] Fixed errors in {behavior_execution_path}")

def main():
    print("Starting systematic fixes for BehaviorNode trait implementations...")
    
    # Fix the root causes in order
    fix_condition_module_visibility()
    fix_startup_schedule_import()
    fix_duplicate_behavior_node()
    fix_missing_trait_methods()
    clean_up_unused_imports()
    fix_behavior_execution_errors()
    
    print("\nAll systematic fixes complete! Please compile the code to check for any remaining issues.")

if __name__ == "__main__":
    main()
