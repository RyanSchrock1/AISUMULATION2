1://! BehaviorNode implementation.
3:use serde;
4:use bevy::prelude::*;
5:use bevy::ecs::entity::Entity;
6:use bevy::ecs::world::World;
7:use super::super::behavior::common::{NodeResult, NodeStatus, BehaviorContext};
8:use super::super::types::*;
9:use std::any::Any;
10:use super::super::behavior::BehaviorNode;
12:use super::super::behavior::common::{NodeResult, NodeStatus, *};
15:use std::sync::Arc;
17:/// Action nodes for behavior trees.
18:///
19:/// This module contains action nodes that perform specific behaviors when executed.
20:/// Actions are the leaf nodes of the behavior tree that actually do something.
24:use rand::Rng;
27:/// Action that makes the AI move to a specific position.
28:/// 
29:/// This action will move the AI towards a target position at a configurable speed.
30:/// It will return `Success` when the AI reaches the target position (within tolerance),
31:/// `Running` while moving, or `Failure` if no target is set or if the target is unreachable.
32:#[derive(Debug, Debug, Debug, Debug, Debug, serde::Serialize, Debug)]
33:#[derive(Debug)]
34:pub struct MoveToPosition {
35:    /// Target position to move to
36:    target: Option<Vec2>,
37:    /// How close the AI needs to get to the target to consider it reached
38:    tolerance: f32,
39:    /// Movement speed multiplier (1.0 = normal speed)
40:    speed: f32,
41:    /// Whether to stop when the target is reached (true) or continue adjusting (false)
42:    stop_at_target: bool,
43:    /// Whether to face the movement direction
44:    face_movement: bool,
45:    /// Internal state for path following
46:    path: Option<Vec<Vec2>>,
47:    current_path_index: usize,
48:}
50:impl MoveToPosition {
51:    /// Creates a new MoveToPosition action.
52:    pub fn new() -> Self {
53:        Self {
54:            target: None,
55:            tolerance: 5.0,
56:            speed: 1.0,
57:            stop_at_target: true,
58:            face_movement: true,
59:            path: None,
60:            current_path_index: 0,
61:        }
62:    }
63:    
64:    /// Sets the target position.
65:    pub fn with_target(mut self, target: Vec2) -> Self {
66:        self.target = Some(target);
67:        self.path = None; // Reset path when target changes
68:        self
69:    }
70:    
71:    /// Sets the movement speed multiplier.
72:    pub fn with_speed(mut self, speed: f32) -> Self {
73:        self.speed = speed.max(0.1); // Ensure speed is always positive
74:        self
75:    }
76:    
77:    /// Sets whether to stop when reaching the target.
78:    pub fn with_stop_at_target(mut self, stop: bool) -> Self {
79:        self.stop_at_target = stop;
80:        self
81:    }
82:    
83:    /// Sets whether to face the movement direction.
84:    pub fn with_face_movement(mut self, face: bool) -> Self {
85:        self.face_movement = face;
86:        self
87:    }
88:    
89:    /// Sets a path to follow instead of a direct line to the target.
90:    pub fn with_path(mut self, path: Vec<Vec2>) -> Self {
91:        if !path.is_empty() {
92:            self.path = Some(path);
93:            self.current_path_index = 0;
94:        }
95:        self
96:    }
97:    
98:    /// Gets the current target position, considering path following.
99:    fn get_current_target(&self) -> Option<Vec2> {
100:        if let Some(ref path) = self.path {
101:            path.get(self.current_path_index).copied()
102:        } else {
103:            self.target
104:        }
105:    }
106:    
107:    /// Updates the path index when reaching waypoints.
108:    fn update_path_index(&mut self, current_pos: Vec2) -> bool {
109:        if let Some(ref path) = &self.path {
110:            let current_target = path[self.current_path_index];
111:            let distance = current_pos.distance(current_target);
112:            
113:            if distance <= self.tolerance && self.current_path_index < path.len() - 1 {
114:                self.current_path_index += 1;
115:                return true;
116:            }
117:        }
118:        false
119:    }
120:}
122:impl BehaviorNode for MoveToPosition {
123:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
124:        let position = context.position;
125:        
126:        // Get the current target (considering path following if applicable)
127:        let target = match self.get_current_target() {
128:            Some(target) => target,
129:            None => return super::super::behavior::common::NodeResult::failure_with_msg("No target position"),
130:        };
131:        
132:        // Check if we've reached the target
133:        let distance = position.distance(target);
134:        if distance <= self.tolerance {
135:            // If we're following a path, move to the next waypoint
136:            if self.update_path_index(position) {
137:                return NodeResult::running_with_msg("Moving to next waypoint");
138:            } else if self.stop_at_target {
139:                // We've reached the final target
140:                return NodeResult::success_with_msg("Reached target position");
141:            }
142:        }
143:        
144:        // Calculate direction to target and move
145:        let direction = (target - position).normalize_or_zero();
146:        let movement = direction * context.delta_time * self.speed * 100.0; // 100.0 is base speed
147:        
148:        // Update position
149:        context.position += movement;
150:        
151:        // Update facing direction if enabled
152:        if self.face_movement && movement.length_squared() > 0.0 {
153:            context.rotation = direction.y.atan2(direction.x);
154:        }
155:        
156:        NodeResult::running()
157:    }
158:    
159:    fn reset(&mut self) {
160:        self.current_path_index = 0;
161:        // Don't clear target or path on reset to allow reuse
162:    }
163:    
164:    fn as_any(&self) -> &dyn std::any::Any {
165:        self
166:    }
167:    
168:    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
169:        self
170:    }
171:    
172:    fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
173:        Box::new(Self {
174:            target: self.target,
175:            tolerance: self.tolerance,
176:            speed: self.speed,
177:            stop_at_target: self.stop_at_target,
178:            face_movement: self.face_movement,
179:            path: self.path.clone(),
180:            current_path_index: self.current_path_index,
181:        })
182:    }
183:}
185:/// Action that makes the AI wander around randomly.
186:#[derive(Debug)]
187:pub struct Wander {
188:    /// Maximum distance the AI can wander from its starting position
189:    pub range: f32,
190:    /// Current target position to wander to
191:    pub target: Option<Vec2>,
192:    /// Last time the AI chose a new wander target
193:    pub last_wander_time: f32,
194:    /// Delay between choosing new wander targets
195:    pub wander_delay: f32,
196:    /// Jitter to add to wander direction
197:    pub jitter: f32,
198:}
200:impl Wander {
201:    /// Creates a new Wander action with the given range.
202:    pub fn new(range: f32) -> Self {
203:        let mut rng = rand::thread_rng();
204:        Self {
205:            range,
206:            target: None,
207:            last_wander_time: 0.0,
208:            wander_delay: 2.0 + rng.gen_range(-0.5..0.5), // Add some randomness to wander timing
209:            jitter: 0.3, // 30% jitter in wander direction
210:        }
211:    }
212:    
213:    /// Chooses a new random target position within wander range
214:    fn choose_new_target(&mut self, position: Vec2) -> Vec2 {
215:        let mut rng = rand::thread_rng();
216:        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
217:        let distance = rng.gen_range(0.0..self.range);
218:        let jitter_x = rng.gen_range(-self.jitter..self.jitter) * self.range;
219:        let jitter_y = rng.gen_range(-self.jitter..self.jitter) * self.range;
220:        
221:        position + Vec2::new(
222:            angle.cos() * distance + jitter_x,
223:            angle.sin() * distance + jitter_y,
224:        )
225:    }
226:}
228:impl BehaviorNode for Wander {
229:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
230:        // Check if we need a new wander target
231:        if self.target.is_none() || 
232:           context.current_time - self.last_wander_time > self.wander_delay {
233:            self.target = Some(self.choose_new_target(context.ai_data.position));
234:            self.last_wander_time = context.current_time;
235:        }
236:        
237:        // Move towards the wander target
238:        let position = context.ai_data.position;
239:        let target = match self.target {
240:            Some(target) => target,
241:            None => return NodeResult::failure_with_msg("No wander target"),
242:        };
243:        
244:        // Check if we've reached the target
245:        if position.distance_squared(target) <= 1.0 {
246:            self.target = Some(self.choose_new_target(position));
247:            return NodeResult::success_with_msg("Reached wander target");
248:        }
249:        
250:        // Calculate direction to target
251:        let direction = (target - position).normalize_or_zero();
252:        
253:        // Calculate new position
254:        let new_position = position + direction * context.ai_data.speed * context.delta_time;
255:        
256:        // Update position and rotation
257:        context.ai_data.position = new_position.into();
258:        context.ai_data.rotation = direction.y.atan2(direction.x);
259:        
260:        NodeResult::running_with_msg("Wandering")
261:    }
263:    fn reset(&mut self) {
264:        self.target = None;
265:        self.last_wander_time = 0.0;
266:    }
268:    
269:    fn as_any(&self) -> &dyn Any {
270:        self
271:    }
272:    
273:    fn as_any_mut(&mut self) -> &mut dyn Any {
274:        self
275:    }
277:            fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
278:        Box::new(Self {
279:            item_id: self.item_id.clone(),
280:            target: self.target,
281:            executed: self.executed,
282:        })
283:    }
284:}
286:/// Action that makes the AI attack its target.
287:#[derive(Debug)]
288:pub struct AttackTarget {
289:    /// Base damage dealt by the attack
290:    pub damage: f32,
291:    /// Maximum attack range
292:    pub range: f32,
293:    /// Cooldown between attacks in seconds
294:    pub cooldown: f32,
295:    /// Time when the last attack occurred
296:    pub last_attack_time: f32,
297:    /// Critical hit chance (0.0 to 1.0)
298:    pub crit_chance: f32,
299:    /// Critical hit multiplier
300:    pub crit_multiplier: f32,
301:}
303:impl AttackTarget {
304:    /// Creates a new AttackTarget action.
305:    pub fn new() -> Self {
306:        Self {
307:            damage: 10.0,
308:            range: 50.0,
309:            cooldown: 1.0,
310:            last_attack_time: -1.0,
311:            crit_chance: 0.1,
312:            crit_multiplier: 2.0,
313:        }
314:    }
315:    
316:    /// Sets the damage amount.
317:    pub fn with_damage(mut self, damage: f32) -> Self {
318:        self.damage = damage;
319:        self
320:    }
321:    
322:    /// Sets the attack range.
323:    pub fn with_range(mut self, range: f32) -> Self {
324:        self.range = range;
325:        self
326:    }
327:    
328:    /// Sets the attack cooldown.
329:    pub fn with_cooldown(mut self, cooldown: f32) -> Self {
330:        self.cooldown = cooldown;
331:        self
332:    }
333:}
335:impl BehaviorNode for AttackTarget {
336:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
337:        // Check if we have a target
338:        let target_entity = match context.target_entity {
339:            Some(entity) => entity,
340:            None => return NodeResult::failure_with_msg("No target to attack"),
341:        };
342:        
343:        // Check if target is in range
344:        let target_position = match context.get_target_position() {
345:            Some(pos) => pos,
346:            None => return NodeResult::failure_with_msg("Target has no position"),
347:        };
348:        
349:        let distance = context.ai_data.position.distance_squared(target_position);
350:        
351:        if distance > self.range * self.range {
352:            return NodeResult::failure_with_msg("Target out of range");
353:        }
354:        
355:        // Check cooldown
356:        if context.current_time - self.last_attack_time < self.cooldown {
357:            return NodeResult::running_with_msg("Attack on cooldown");
358:        }
359:        
360:        // Perform attack
361:        self.last_attack_time = context.current_time;
362:        
363:        // In a real game, you would apply damage to the target here
364:        // For now, we'll just return success
365:        NodeResult::success_with_msg("Attacked target")
366:    }
367:    
368:    fn reset(&mut self) {
369:        self.last_attack_time = -1.0;
370:    }
371:    
372:    fn as_any(&self) -> &dyn std::any::Any {
373:        self
374:    }
375:    
376:    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
377:        self
378:    }
380:            fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
381:        Box::new(Self {
382:            item_id: self.item_id.clone(),
383:            target: self.target,
384:            executed: self.executed,
385:        })
386:    }
387:}
389:/// Action node that makes the AI wait for a specified duration.
390:#[derive(Debug)]
391:pub struct Wait {
392:    duration: f32,
393:    start_time: Option<f32>,
394:}
396:impl Wait {
397:    /// Creates a new Wait action.
398:    pub fn new(duration: f32) -> Self {
399:        Self {
400:            duration,
401:            start_time: None,
402:        }
403:    }
404:}
406:impl BehaviorNode for Wait {
407:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
408:        let current_time = context.time;
409:        
410:        // Initialize start time if this is the first execution
411:        if self.start_time.is_none() {
412:            self.start_time = Some(current_time);
413:            return super::super::behavior::common::NodeResult::running_with_msg("Starting wait");
414:        }
415:        
416:        // Check if we've waited long enough
417:        if current_time - self.start_time.unwrap() >= self.duration {
418:            self.reset();
419:            super::super::behavior::common::NodeResult::success_with_msg("Finished waiting")
420:        } else {
421:            super::super::behavior::common::NodeResult::running_with_msg("Waiting...")
422:        }
423:    }
424:    
425:    fn reset(&mut self) {
426:        self.start_time = None;
427:    }
428:    
429:    fn as_any(&self) -> &dyn std::any::Any {
430:        self
431:    }
432:    
433:    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
434:        self
435:    }
436:    
437:    fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
438:        Box::new(Self {
439:            duration: self.duration,
440:            start_time: self.start_time,
441:        })
442:    }
443:}
445:/// Action that makes the AI flee from a threat.
446:/// 
447:/// This action will move the AI away from a specified threat position at maximum speed.
448:#[derive(Debug)]
449:pub struct FleeFromThreat {
450:    /// The position of the threat to flee from
451:    threat_position: Option<Vec2>,
452:    /// How close the threat needs to be to trigger fleeing
453:    detection_range: f32,
454:    /// How far to move away from the threat
455:    flee_distance: f32,
456:    /// Current flee target (cached to avoid recalculating)
457:    flee_target: Option<Vec2>,
458:}
460:impl FleeFromThreat {
461:    /// Creates a new FleeFromThreat action.
462:    pub fn new(detection_range: f32, flee_distance: f32) -> Self {
463:        Self {
464:            threat_position: None,
465:            detection_range: detection_range.max(0.0),
466:            flee_distance: flee_distance.max(0.0),
467:            flee_target: None,
468:        }
469:    }
470:    
471:    /// Sets the threat position to flee from.
472:    pub fn with_threat_position(mut self, position: Vec2) -> Self {
473:        self.threat_position = Some(position);
474:        self.flee_target = None; // Reset flee target
475:        self
476:    }
477:    
478:    /// Calculates a position to flee to.
479:    fn calculate_flee_target(&self, position: Vec2) -> Option<Vec2> {
480:        self.threat_position.map(|threat_pos| {
481:            let direction = (position - threat_pos).normalize_or_zero();
482:            position + direction * self.flee_distance
483:        })
484:    }
485:}
487:impl BehaviorNode for FleeFromThreat {
488:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
489:        let position = context.position;
490:        
491:        // If we don't have a threat position, try to get it from context
492:        if self.threat_position.is_none() {
493:            // In a real implementation, you might get this from perception or blackboard
494:            return super::super::behavior::common::NodeResult::failure_with_msg("No threat position");
495:            fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
496:        Box::new(Self {
497:            threat_position: self.threat_position,
498:            detection_range: self.detection_range,
499:            flee_distance: self.flee_distance,
500:            flee_target: self.flee_target,
501:        })
502:    }
503:}
504:        
505:        // Calculate distance to threat
506:        let threat_pos = self.threat_position.unwrap();
507:        let distance_to_threat = position.distance(threat_pos);
508:        
509:        // If threat is too far away, we're done
510:        if distance_to_threat > self.detection_range {
511:            self.flee_target = None;
512:            return super::super::behavior::common::NodeResult::success_with_msg("Threat out of range");
513:        }
514:        
515:        // If we don't have a flee target or it's too close to the threat, calculate a new one
516:        if self.flee_target.is_none() || 
517:           self.flee_target.map_or(false, |t| t.distance(threat_pos) < self.detection_range * 1.5) {
518:            self.flee_target = self.calculate_flee_target(position);
519:        }
520:        
521:        // Move away from the threat
522:        if let Some(flee_target) = self.flee_target {
523:            // Use MoveToPosition to handle the actual movement
524:            let mut move_action = MoveToPosition::new()
525:                .with_target(flee_target)
526:                .with_speed(1.5) // Flee at 1.5x normal speed
527:                .with_stop_at_target(false)
528:                .with_face_movement(true);
529:                
530:            let result = move_action.execute(context);
531:            
532:            // If we've reached our flee target but the threat is still too close, pick a new target
533:            if result.is_success() && distance_to_threat < self.detection_range * 1.5 {
534:                self.flee_target = self.calculate_flee_target(position);
535:                return super::super::behavior::common::NodeResult::running_with_msg("Moving to next waypoint");
536:            }
537:            
538:            result
539:        } else {
540:            super::super::behavior::common::NodeResult::failure_with_msg("Failed to calculate flee target")
541:        }
542:    }
543:    
544:    fn reset(&mut self) {
545:        self.flee_target = None;
546:    }
547:    
548:    fn as_any(&self) -> &dyn Any {
549:        self
550:    }
551:    
552:    fn as_any_mut(&mut self) -> &mut dyn Any {
553:        self
554:    }
556:            fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
557:        Box::new(Self {
558:            item_id: self.item_id.clone(),
559:            target: self.target,
560:            executed: self.executed,
561:        })
562:    }
563:}
565:/// Action that makes the AI heal itself or an ally.
566:#[derive(Debug)]
567:pub struct Heal {
568:    /// Amount of health to restore per second
569:    heal_rate: f32,
570:    /// Maximum health to restore (None for no limit)
571:    max_heal: Option<f32>,
572:    /// Total amount healed so far
573:    amount_healed: f32,
574:    /// Target entity to heal (None for self)
575:    target_entity: Option<Entity>,
576:}
578:impl Heal {
579:    /// Creates a new Heal action.
580:    pub fn new(heal_rate: f32) -> Self {
581:        Self {
582:            heal_rate: heal_rate.max(0.0),
583:            max_heal: None,
584:            amount_healed: 0.0,
585:            target_entity: None,
586:        }
587:    }
588:    
589:    /// Sets the maximum amount of health to restore.
590:    pub fn with_max_heal(mut self, max_heal: f32) -> Self {
591:        self.max_heal = Some(max_heal.max(0.0));
592:        self
593:    }
594:    
595:    /// Sets the target entity to heal.
596:    pub fn with_target(mut self, entity: Entity) -> Self {
597:        self.target_entity = Some(entity);
598:        self
599:    }
600:}
602:impl BehaviorNode for Heal {
603:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
604:        // Calculate heal amount for this frame
605:        let heal_amount = self.heal_rate * context.delta_time;
606:        
607:        // Check if we've reached the maximum heal amount
608:        if let Some(max_heal) = self.max_heal {
609:            if self.amount_healed >= max_heal {
610:                return super::super::behavior::common::NodeResult::success_with_msg("Maximum heal amount reached");
611:            }
612:            
613:            // Don't exceed max heal
614:            let remaining_heal = max_heal - self.amount_healed;
615:            let actual_heal = heal_amount.min(remaining_heal);
616:            
617:            // Apply healing
618:            context.health = (context.health + actual_heal).min(1.0);
619:            self.amount_healed += actual_heal;
620:            
621:            if self.amount_healed >= max_heal {
622:                return super::super::behavior::common::NodeResult::success_with_msg("Healing complete");
623:            }
624:            
625:            super::super::behavior::common::NodeResult::running_with_msg(format!("Healing: {:.1}%", self.amount_healed * 100.0 / max_heal))
626:        } else {
627:            // No max heal, just heal continuously
628:            context.health = (context.health + heal_amount).min(1.0);
629:            self.amount_healed += heal_amount;
630:            super::super::behavior::common::NodeResult::running_with_msg("Healing in progress")
631:        }
632:    }
633:    fn reset(&mut self) {
634:        self.amount_healed = 0.0;
635:    }
636:    
637:    fn as_any(&self) -> &dyn Any {
638:        self
639:    }
640:    
641:    fn as_any_mut(&mut self) -> &mut dyn Any {
642:        self
643:    }
644:    
645:    fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
646:        Box::new(Self {
647:            heal_rate: self.heal_rate,
648:            max_heal: self.max_heal,
649:            amount_healed: self.amount_healed,
650:            target_entity: self.target_entity,
651:        })
652:    }
653:}
655:/// Action that makes the AI use an item or ability.
656:#[derive(Debug)]
657:pub struct UseItem {
658:    /// ID of the item or ability to use
659:    item_id: String,
660:    /// Target entity (if any)
661:    target: Option<Entity>,
662:    /// Whether the action has been executed
663:    executed: bool,
664:}
666:impl UseItem {
667:    /// Creates a new UseItem action.
668:    pub fn new(item_id: &str) -> Self {
669:        Self {
670:            item_id: item_id.to_string(),
671:            target: None,
672:            executed: false,
673:        }
674:    }
675:    
676:    /// Sets the target entity for the item.
677:    pub fn with_target(mut self, target: Entity) -> Self {
678:        self.target = Some(target);
679:        self
680:    }
681:}
683:impl BehaviorNode for UseItem {
684:    fn execute(&mut self, context: &mut BehaviorContext) -> NodeResult {
685:        if self.executed {
686:            return super::super::behavior::common::NodeResult::success_with_msg("Item already used");
687:        }
688:        
689:        // In a real implementation, you would:
690:        // 1. Check if the item exists in the AI's inventory
691:        // 2. Check if the item can be used on the target (if any)
692:        // 3. Apply the item's effects
693:        // 4. Remove the item if it's consumable
694:        
695:        self.executed = true;
696:        
697:        if let Some(target) = self.target {
698:            super::super::behavior::common::NodeResult::success_with_msg(format!("Used {} on {:?}", self.item_id, target))
699:        } else {
700:            super::super::behavior::common::NodeResult::success_with_msg(format!("Used {}", self.item_id))
701:        }
702:    }
703:    
704:    fn reset(&mut self) {
705:        self.executed = false;
706:    }
707:    
708:    fn as_any(&self) -> &dyn Any {
709:        self
710:    }
711:    
712:    fn as_any_mut(&mut self) -> &mut dyn Any {
713:        self
714:    }
716:            fn clone_boxed(&self) -> Box<dyn BehaviorNode> {
717:        Box::new(Self {
718:            item_id: self.item_id.clone(),
719:            target: self.target,
720:            executed: self.executed,
721:        })
722:    }
723:}
