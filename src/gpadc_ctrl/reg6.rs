#[doc = "Register `REG6` reader"]
pub type R = crate::R<Reg6Spec>;
#[doc = "Register `REG6` writer"]
pub type W = crate::W<Reg6Spec>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP8` reader - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp8R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP8` writer - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP9` reader - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp9R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP9` writer - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP10` reader - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp10R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP10` writer - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP11` reader - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp11R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP11` writer - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp8(&self) -> PktRamBaseAddrCp8R {
        PktRamBaseAddrCp8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp9(&self) -> PktRamBaseAddrCp9R {
        PktRamBaseAddrCp9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp10(&self) -> PktRamBaseAddrCp10R {
        PktRamBaseAddrCp10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp11(&self) -> PktRamBaseAddrCp11R {
        PktRamBaseAddrCp11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp8(&mut self) -> PktRamBaseAddrCp8W<Reg6Spec> {
        PktRamBaseAddrCp8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp9(&mut self) -> PktRamBaseAddrCp9W<Reg6Spec> {
        PktRamBaseAddrCp9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp10(&mut self) -> PktRamBaseAddrCp10W<Reg6Spec> {
        PktRamBaseAddrCp10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp11(&mut self) -> PktRamBaseAddrCp11W<Reg6Spec> {
        PktRamBaseAddrCp11W::new(self, 24)
    }
}
#[doc = "Base address for Chirp profile 2 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg6Spec;
impl crate::RegisterSpec for Reg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg6::R`](R) reader structure"]
impl crate::Readable for Reg6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg6::W`](W) writer structure"]
impl crate::Writable for Reg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG6 to value 0"]
impl crate::Resettable for Reg6Spec {
    const RESET_VALUE: u32 = 0;
}
