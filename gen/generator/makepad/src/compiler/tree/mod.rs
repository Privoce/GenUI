mod traits;

pub use traits::*;

use std::{
    collections::HashSet,
    hash::Hash,
    path::{Path, PathBuf},
};

/// ## 多叉模型树(FS Tree)
/// 用于表示一个文件系统树，这个树是一个多叉树，除了根节点外，每个节点都是都会是一个文件
/// 除了根节点外，后续的节点的Path其实都是文件的Path
///
/// ### struct example
/// ```
/// {
/// node: /path/to/project/src, (tree root 永远是整个项目的src目录)
/// children: [
///     {node: src/a.gen, children: [
///             {node: src/views/b.gen, children: None},
///             {node: src/views/d.gen, children: None},
///             {node: src/components/c.gen, children: None}
///         ]
///     },
///   ]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ModelTree {
    pub node: PathBuf,
    pub children: Option<HashSet<ModelTree>>,
}

impl PartialEq for ModelTree {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for ModelTree {}

impl Hash for ModelTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

impl ModelTree {
    /// ## get all registers from tree
    /// 实际上ModelTree就是一个以注册组件构建而成的树，所以直接调用expand方法就可以得到所有的注册组件，然后去除根节点即可
    pub fn registers(&self) -> HashSet<String> {
        self.expand_map(
            |p| p != self.node.as_path() && p.file_stem().unwrap() != "mod",
            |p| {
                p.with_extension("")
                    .strip_prefix(self.node.as_path())
                    .unwrap()
                    .components()
                    .map(|item| item.as_os_str().to_str().unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join("::")
            },
        )
    }
    /// ## expand model tree
    /// expand model tree to path set
    pub fn expand(&self) -> HashSet<PathBuf> {
        self.expand_map(|_| true, |p| p.to_path_buf())
    }
    /// ## expand model tree with filter-map
    pub fn expand_map<R, F, M>(&self, filter: F, map: M) -> HashSet<R>
    where
        R: Hash + Eq,
        F: Fn(&Path) -> bool,
        M: Fn(&Path) -> R,
    {
        fn handle<R, F, M>(node: &ModelTree, paths: &mut HashSet<R>, filter: &F, map: &M) -> ()
        where
            R: Hash + Eq,
            F: Fn(&Path) -> bool,
            M: Fn(&Path) -> R,
        {
            if filter(node.node.as_path()) {
                // do map and insert

                let _ = paths.insert(map(node.node.as_path()));
            }

            if let Some(children) = node.children.as_ref() {
                for child in children {
                    handle(child, paths, filter, map);
                }
            }
        }

        let mut paths = HashSet::new();

        handle(self, &mut paths, &filter, &map);

        paths
    }

    pub fn lib_rs(&self) -> Option<HashSet<PathBuf>> {
        if let Some(nodes) = self.get(1) {
            // use get can get all nodes from level but lib should output dir
            Some(nodes.iter().fold(HashSet::new(), |mut acc, node| {
                let mut node = node.clone();
                let _ = node.pop();
                let node = node
                    .strip_prefix(self.node.as_path())
                    .unwrap()
                    .to_path_buf();

                if acc.get(&node).is_none() {
                    let _ = acc.insert(node);
                }

                acc
            }))
        } else {
            None
        }
    }

    /// ## get model tree from level
    pub fn get(&self, level: usize) -> Option<HashSet<PathBuf>> {
        let root_level = self.level();

        fn handle(
            node: &ModelTree,
            nodes: &mut HashSet<PathBuf>,
            root_level: usize,
            level: usize,
        ) -> () {
            let current_level = node.level();
            let strip_level = (current_level - root_level) as isize - level as isize;

            if strip_level == 0 {
                let _ = nodes.insert(node.node.clone());
            } else if strip_level < 0 {
                // do nested
                if let Some(children) = node.children.as_ref() {
                    for child in children {
                        let _ = handle(child, nodes, root_level, level);
                    }
                }
            }
        }

        let mut nodes = HashSet::new();
        let _ = handle(self, &mut nodes, root_level, level);

        if nodes.is_empty() {
            None
        } else {
            Some(nodes)
        }
    }

    /// ## get model tree node level
    pub fn level(&self) -> usize {
        let mut node = self.node.clone();
        if node.extension().is_some() {
            let _ = node.pop();
        }
        node.components().count()
    }

    /// create new model tree
    pub fn new<P>(node: P) -> ModelTree
    where
        P: AsRef<Path>,
    {
        Self {
            node: node.as_ref().to_path_buf(),
            children: None,
        }
    }
    // pub fn root_live_register(&self) -> String {
    //     format!(
    //         "crate::{}::live_design(cx);",
    //         self.node.source().unwrap().to_live_register()
    //     )
    // }
    // /// get node from tree
    // pub fn get<P>(&self, key: P) -> Option<&> where P: AsRef<Path> {
    //     if self.node == key.as_ref() {
    //         return Some(&self);
    //     }
    //     if let Some(children) = &self.children {
    //         for child in children {
    //             if let Some(node) = child.get(key) {
    //                 return Some(node);
    //             }
    //         }
    //         // return children.iter().find_map(|child| child.get(key));
    //     }
    //     None
    // }
    /// insert node to widget tree
    /// compare path, src is the same root
    /// eg:
    /// - item_path:  src/a1/b/c
    /// - current_path: src/a2
    /// means: item should in 4 level
    pub fn insert<P>(&mut self, item: P) -> ()
    where
        P: AsRef<Path>,
    {
        fn similarity<P>(path1: P, path2: P) -> usize
        where
            P: AsRef<Path>,
        {
            let components1: Vec<_> = path1.as_ref().components().collect();
            let components2: Vec<_> = path2.as_ref().components().collect();

            components1
                .iter()
                .zip(components2.iter())
                .take_while(|(a, b)| a == b)
                .count()
        }
        let is_root = self.node.eq(item.as_ref());
        // get level and compare
        let item_level = item.as_ref().level();

        if let Some(children) = &mut self.children {
            // let mut is_root = true;
            // 查找子节点中任意的path的节点，首先使用level匹配，level相同，可以直接push
            // level不同，若当前level比item的level小，继续遍历子节点，大则将当前children放到item的children中，再把item放回父节点进行替换
            // let (current_level, _current_path) = children[0].level();
            let current_level = children.iter().next().unwrap().node.level();
            let step = item_level as isize - current_level as isize;

            if step == 0 {
                let node: ModelTree = item.as_ref().into();
                if is_root {
                    self.node = node.node;
                } else {
                    let _ = children.remove(&node);
                    let _ = children.insert(node);
                }
            } else if step < 0 {
                // 说明item节点比当前节点层级高，将item节点替换当前的节点
                let mut node: ModelTree = item.as_ref().into();
                node.children.replace(self.children.take().unwrap());
                // add into parent node
                // let _ = std::mem::replace(&mut self.children, Some(vec![node]));
                let _ =
                    std::mem::replace(&mut self.children, Some(std::iter::once(node).collect()));
            } else {
                // 说明item节点比当前节点层级低，继续遍历子节点
                // 需要查找当前所有子节点的path，找到符合前缀的节点，查看子节点数量，哪个少往哪个去遍历（符合前缀指的是前缀匹配优先级最大的）
                // 不能使用start_with去匹配，因为无法知道若前缀没有完全相同的情况下的优先级长度
                // 例如： [src/a/z/y]
                // 1. src/a/b/c , 2. src/a/z , 3. src/a/z/y
                // 那么应该选择第三个节点进行遍历，因为第三个节点的前缀匹配优先级最大
                // 递归调用当前这个方法
                let mut target_node: Option<ModelTree> = None;
                let mut max_sim = 0_usize;
                for child in children.iter() {
                    // let (_, child_path) = child.level();

                    // compare child path and item path
                    let sim = similarity(item.as_ref(), child.node.as_path());
                    if sim.eq(&0_usize) {
                        // 相似度为0，说明没有相同的前缀，直接跳过
                        continue;
                    } else {
                        // 有相似度，和当前max相似度比较, 大于max则替换target_node
                        if sim.gt(&max_sim) {
                            max_sim = sim;
                            target_node.replace(child.clone());
                        }
                    }
                }
                // 查看target_node是否存在，存在说明找到了优先级最大的节点，递归调用这个add方法，不存在则直接push
                let target_node = if let Some(mut target_node) = target_node {
                    children.remove(&target_node);
                    target_node.insert(item);
                    target_node
                } else {
                    // children.push(item.into());
                    item.as_ref().into()
                };

                let _ = children.insert(target_node);
            }
        } else {
            // now have no children, just set
            // self.children.replace(vec![item.into()]);
            self.children = Some(std::iter::once(item.as_ref().into()).collect());
        }
    }
    pub fn get_node<P>(&self, key: P) -> Option<ModelTree>
    where
        P: AsRef<Path>,
    {
        if self.node == key.as_ref() {
            return Some(self.clone());
        }
        if let Some(children) = &self.children {
            for child in children {
                if let Some(node) = child.get_node(key.as_ref()) {
                    return Some(node.clone());
                }
            }
            // return children.iter().find_map(|child| child.get(key));
        }
        None
    }

    /// remove node from tree
    /// 移除节点，这里不能像get_node一样一层层进去，因为remove的时候需要找到父节点，然后将父节点的children中的子节点移除
    /// 所以这里需要使用递归的方式，找到父节点，然后将父节点的children中的子节点移除
    /// 但基于HashSet的特性并且我只要移除一个节点，它的子节点依然需要
    pub fn remove<P>(&mut self, key: P) -> Option<bool>
    where
        P: AsRef<Path>,
    {
        if key.as_ref() == self.node.as_path() {
            self.children = None;
            return Some(true);
        }

        if let Some(node) = self.get_node(key.as_ref()) {
            if let Some(children) = self.children.as_mut() {
                if children.remove(&ModelTree::new(key.as_ref())) {
                    // do insert
                    if let Some(children) = node.children.as_ref() {
                        for child in children {
                            self.insert(child.node.as_path());
                        }
                    }
                    return Some(true);
                }
            }
        }

        None
    }

    //     Ok(())
    // }

    // /// get live register from tree
    // pub fn to_live_register(&self) -> Vec<String> {
    //     // get basic live register => ui widget ref from super ui root
    //     let mut live_register = vec![];
    //     live_register.push(self.node.source().unwrap().to_live_register());
    //     // children
    //     if let Some(children) = self.children.as_ref() {
    //         for child in children {
    //             live_register.extend(child.to_live_register());
    //         }
    //     }
    //     live_register
    // }
    // /// get root import
    // pub fn to_imports(&self) -> TokenStream {
    //     let mut imports = TokenStream::new();
    //     let import_str: TokenStream =
    //         parse_str(&self.node.source().unwrap().to_live_register()).unwrap();
    //     imports.extend(quote! {import crate::#import_str::*;});
    //     imports
    // }
    // /// ## get widget tree level
    // /// tree level can get from node source path
    // /// ### return
    // /// (level, path)
    // /// - `level: usize`: path length which can easy know the level of the tree, if compare with another level can know the tree is child or parent, acturally you can think level is just offset of dir path
    // /// - `path: PathBuf`: level path which only contain dir level
    // pub fn level(&self) -> (usize, PathBuf) {
    //     let source = self.node.source().unwrap().level_gen();

    //     (source.components().count(), source)
    // }
    // pub fn default_root() -> ModelTree {
    //     ModelTree {
    //         node: Widget::default_ui_root().into(),
    //         children: None,
    //     }
    // }
    // /// get super ui root name
    // /// return (root_source_name, root_widget_id)
    // pub fn super_ui_root(&self) -> (String, String) {
    //     // self.node.source().unwrap().source_name_lower()
    //     self.node.super_ui_root()
    // }
    // #[allow(dead_code)]
    // fn lib_debug(&self) -> HashSet<String> {
    //     let mut mods = HashSet::new();
    //     mods.insert(self.node.source().unwrap().to_lib());
    //     if self.children.as_ref().is_some() {
    //         self.children.as_ref().unwrap().iter().for_each(|child| {
    //             mods.extend(child.lib_debug());
    //         });
    //     }
    //     mods
    // }
    // /// convert model tree to lib.rs mod
    // pub fn to_lib(&self) -> TokenStream {
    //     // get node model source
    //     self.to_lib_list()
    //         .iter()
    //         .fold(TokenStream::new(), |mut acc, item| {
    //             let item = token_tree_ident(item);
    //             acc.extend(quote! {
    //                 pub mod #item;
    //             });
    //             acc
    //         })
    // }
    // /// convert model tree to lib.rs mod list
    // /// acutally this method is used to get all mod name
    // /// what need to do is get the first level file name or dir name
    // pub fn to_lib_list(&self) -> Vec<String> {
    //     let mut mods = HashSet::new();

    //     if let Some(children) = &self.children {
    //         for child in children {
    //             let mod_name = child.node.source().unwrap().to_lib();

    //             mods.insert(mod_name);
    //         }
    //     }

    //     mods.into_iter().collect()
    // }
    // /// compile model tree
    // /// 遍历整个树，将每个节点的内容写入到文件中
    // pub fn compile(&self) -> () {
    //     // let loop_tree = |node: &ModelNode| -> () {
    //     //     let content = node.content().to_string();
    //     //     let mut file = create_file(node.source().unwrap().compiled_file.as_path());
    //     //     file.write_all(content.as_bytes()).unwrap();
    //     // };

    //     // let _ = loop_tree(&self.node);
    //     let _ = self.node.compile();
    //     // children
    //     if let Some(children) = self.children.as_ref() {
    //         for child in children {
    //             let _ = child.compile();
    //         }
    //     }
    // }
}

impl From<&Path> for ModelTree {
    fn from(path: &Path) -> Self {
        ModelTree::new(path)
    }
}

#[cfg(test)]
mod test_tree {

    use super::ModelTree;

    #[test]
    fn tree_get() {
        let mut tree = ModelTree{
            node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src".into(),
            children: Some(
                vec![
                    ModelTree{
                        node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/views/login.rs".into(),
                        children: Some(
                            vec![
                                ModelTree{
                                    node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/views/hello/test.rs".into(),
                                    children: None,
                                }
                            ].into_iter().collect()
                        ),
                    },
                    ModelTree{
                        node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/views/root.rs".into(),
                        children: None,
                    },
                    ModelTree{
                        node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/components/1.rs".into(),
                        children: Some(
                            vec![
                                ModelTree{
                                    node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/components/header/header.rs".into(),
                                    children: None,
                                }
                            ].into_iter().collect()
                        ),
                    },
                ].into_iter().collect()
            ),
        };

        // let node0 = tree.get(0);
        // let node1 = tree.get(1);

        // dbg!(node0);
        // dbg!(node1);

        // dbg!(create_lib_rs(tree.lib_rs()).to_string());

        // if let Some(children) = tree.children.as_mut() {
        //     children.remove(&ModelTree{
        //         node: "/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/views/login.rs".into(),
        //         children: None,
        //     });
        // }

        tree.remove("/Users/shengyifei/projects/gen_ui/GenUI/examples/ract/test1/src_gen_0/src/views/login.rs");

        dbg!(tree);
    }
}
