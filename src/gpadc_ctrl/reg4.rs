#[doc = "Register `REG4` reader"]
pub type R = crate::R<Reg4Spec>;
#[doc = "Register `REG4` writer"]
pub type W = crate::W<Reg4Spec>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP0` reader - 7:0\\]
Start Address of instruction-ram in CTM mode"]
pub type PktRamBaseAddrCp0R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP0` writer - 7:0\\]
Start Address of instruction-ram in CTM mode"]
pub type PktRamBaseAddrCp0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP1` reader - 15:8\\]
(End-Address + 1) of instruction-ram in CTM mode"]
pub type PktRamBaseAddrCp1R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP1` writer - 15:8\\]
(End-Address + 1) of instruction-ram in CTM mode"]
pub type PktRamBaseAddrCp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP2` reader - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp2R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP2` writer - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP3` reader - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp3R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP3` writer - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Start Address of instruction-ram in CTM mode"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp0(&self) -> PktRamBaseAddrCp0R {
        PktRamBaseAddrCp0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
(End-Address + 1) of instruction-ram in CTM mode"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp1(&self) -> PktRamBaseAddrCp1R {
        PktRamBaseAddrCp1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp2(&self) -> PktRamBaseAddrCp2R {
        PktRamBaseAddrCp2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp3(&self) -> PktRamBaseAddrCp3R {
        PktRamBaseAddrCp3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Start Address of instruction-ram in CTM mode"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp0(&mut self) -> PktRamBaseAddrCp0W<Reg4Spec> {
        PktRamBaseAddrCp0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
(End-Address + 1) of instruction-ram in CTM mode"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp1(&mut self) -> PktRamBaseAddrCp1W<Reg4Spec> {
        PktRamBaseAddrCp1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp2(&mut self) -> PktRamBaseAddrCp2W<Reg4Spec> {
        PktRamBaseAddrCp2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp3(&mut self) -> PktRamBaseAddrCp3W<Reg4Spec> {
        PktRamBaseAddrCp3W::new(self, 24)
    }
}
#[doc = "Base address for Chirp profile 0 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg4Spec;
impl crate::RegisterSpec for Reg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg4::R`](R) reader structure"]
impl crate::Readable for Reg4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg4::W`](W) writer structure"]
impl crate::Writable for Reg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG4 to value 0"]
impl crate::Resettable for Reg4Spec {
    const RESET_VALUE: u32 = 0;
}
