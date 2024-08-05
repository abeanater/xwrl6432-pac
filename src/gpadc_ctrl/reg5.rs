#[doc = "Register `REG5` reader"]
pub type R = crate::R<Reg5Spec>;
#[doc = "Register `REG5` writer"]
pub type W = crate::W<Reg5Spec>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP4` reader - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp4R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP4` writer - 7:0\\]
TI reserved"]
pub type PktRamBaseAddrCp4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP5` reader - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp5R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP5` writer - 15:8\\]
TI reserved"]
pub type PktRamBaseAddrCp5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP6` reader - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp6R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP6` writer - 23:16\\]
TI reserved"]
pub type PktRamBaseAddrCp6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP7` reader - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp7R = crate::FieldReader;
#[doc = "Field `PKT_RAM_BASE_ADDR_CP7` writer - 31:24\\]
TI reserved"]
pub type PktRamBaseAddrCp7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp4(&self) -> PktRamBaseAddrCp4R {
        PktRamBaseAddrCp4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp5(&self) -> PktRamBaseAddrCp5R {
        PktRamBaseAddrCp5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp6(&self) -> PktRamBaseAddrCp6R {
        PktRamBaseAddrCp6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    pub fn pkt_ram_base_addr_cp7(&self) -> PktRamBaseAddrCp7R {
        PktRamBaseAddrCp7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp4(&mut self) -> PktRamBaseAddrCp4W<Reg5Spec> {
        PktRamBaseAddrCp4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp5(&mut self) -> PktRamBaseAddrCp5W<Reg5Spec> {
        PktRamBaseAddrCp5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp6(&mut self) -> PktRamBaseAddrCp6W<Reg5Spec> {
        PktRamBaseAddrCp6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_ram_base_addr_cp7(&mut self) -> PktRamBaseAddrCp7W<Reg5Spec> {
        PktRamBaseAddrCp7W::new(self, 24)
    }
}
#[doc = "Base address for Chirp profile 1 in instruction packet RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg5Spec;
impl crate::RegisterSpec for Reg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg5::R`](R) reader structure"]
impl crate::Readable for Reg5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg5::W`](W) writer structure"]
impl crate::Writable for Reg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG5 to value 0"]
impl crate::Resettable for Reg5Spec {
    const RESET_VALUE: u32 = 0;
}
