#[doc = "Register `REG7` reader"]
pub type R = crate::R<Reg7Spec>;
#[doc = "Register `REG7` writer"]
pub type W = crate::W<Reg7Spec>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP12` reader - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp12R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP12` writer - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP13` reader - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp13R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP13` writer - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP14` reader - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp14R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP14` writer - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP15` reader - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp15R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP15` writer - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp12(&self) -> PktRamBaseAddrCp12R {
        PktRamBaseAddrCp12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp13(&self) -> PktRamBaseAddrCp13R {
        PktRamBaseAddrCp13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp14(&self) -> PktRamBaseAddrCp14R {
        PktRamBaseAddrCp14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp15(&self) -> PktRamBaseAddrCp15R {
        PktRamBaseAddrCp15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp12(&mut self) -> PktRamBaseAddrCp12W<Reg7Spec> {
        PktRamBaseAddrCp12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp13(&mut self) -> PktRamBaseAddrCp13W<Reg7Spec> {
        PktRamBaseAddrCp13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp14(&mut self) -> PktRamBaseAddrCp14W<Reg7Spec> {
        PktRamBaseAddrCp14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp15(&mut self) -> PktRamBaseAddrCp15W<Reg7Spec> {
        PktRamBaseAddrCp15W::new(self, 24)
    }
}
#[doc = "Base address for Chirp profile 3 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg7Spec;
impl crate::RegisterSpec for Reg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg7::R`](R) reader structure"]
impl crate::Readable for Reg7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg7::W`](W) writer structure"]
impl crate::Writable for Reg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG7 to value 0"]
impl crate::Resettable for Reg7Spec {
    const RESET_VALUE: u32 = 0;
}
